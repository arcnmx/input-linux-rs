//! An interface to the Linux uinput kernel module that can be used to create
//! virtual input devices.

use std::{io, fs, ptr};
use std::os::unix::io::{RawFd, AsRawFd, IntoRawFd, FromRawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::raw::c_char;
use std::mem::{MaybeUninit, size_of};
use std::path::{Path, PathBuf};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::ffi::{OsStr, OsString, CStr};
use sys;
use nix;
use ::{InputId, AbsoluteInfoSetup, kinds};
use macros::convert_error;

pub use sys::UINPUT_VERSION;
pub use sys::UINPUT_MAX_NAME_SIZE;

/// A handle to a uinput allowing the use of ioctls
pub struct UInputHandle<F>(F);

fn copy_name(dest: &mut [c_char; UINPUT_MAX_NAME_SIZE as usize], name: &[u8]) -> io::Result<()> {
    if name.len() >= UINPUT_MAX_NAME_SIZE as usize {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "name too long"))
    } else {
        unsafe {
            ptr::copy_nonoverlapping(name.as_ptr() as *const _, dest.as_mut_ptr() as *mut _, name.len());
        }
        dest[name.len()] = 0 as _;

        Ok(())
    }
}

impl<F> UInputHandle<F> {
    /// Create a new handle using an existing open file object.
    pub fn new(fd: F) -> Self {
        UInputHandle(fd)
    }

    /// Extracts the contained handle.
    pub fn into_inner(self) -> F {
        self.0
    }

    /// A reference to the contained handle.
    pub fn as_inner(&self) -> &F {
        &self.0
    }

    /// A mutable reference to the contained handle.
    pub fn as_inner_mut(&mut self) -> &mut F {
        &mut self.0
    }
}

impl<F: IntoRawFd> IntoRawFd for UInputHandle<F> {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl<F: FromRawFd> FromRawFd for UInputHandle<F> {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        UInputHandle(FromRawFd::from_raw_fd(fd))
    }
}

impl UInputHandle<fs::File> {
    /// Create a new handle from a raw file descriptor.
    pub unsafe fn from_fd(fd: RawFd) -> Self {
        FromRawFd::from_raw_fd(fd)
    }
}

impl<F: AsRawFd> UInputHandle<F> {
    #[inline]
    fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    /// Create a new uinput device using the legacy `UI_DEV_CREATE` interface
    pub fn create_legacy(&self, id: &InputId, name: &[u8], ff_effects_max: u32, abs: &[AbsoluteInfoSetup]) -> io::Result<()> {
        let mut setup: sys::uinput_user_dev = unsafe { MaybeUninit::zeroed().assume_init() };
        setup.id = (*id).into();
        setup.ff_effects_max = ff_effects_max;

        copy_name(&mut setup.name, name)?;

        for abs in abs {
            let code = abs.axis as usize;
            if code >= sys::ABS_CNT as _ {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid abs axis code"))
            }

            let abs = &abs.info;
            setup.absmax[code] = abs.maximum;
            setup.absmin[code] = abs.minimum;
            setup.absfuzz[code] = abs.fuzz;
            setup.absflat[code] = abs.flat;
        }

        let setup = unsafe { from_raw_parts(&setup as *const _ as *const u8, size_of::<sys::uinput_user_dev>()) };
        nix::unistd::write(self.fd(), setup).map_err(convert_error)?;

        self.dev_create()
    }

    /// Create a new uinput device, and fall back on the legacy interface if necessary
    pub fn create(&self, id: &InputId, name: &[u8], ff_effects_max: u32, abs: &[AbsoluteInfoSetup]) -> io::Result<()> {
        let mut setup: sys::uinput_setup = unsafe { MaybeUninit::zeroed().assume_init() };
        setup.id = (*id).into();
        setup.ff_effects_max = ff_effects_max;

        copy_name(&mut setup.name, name)?;

        match self.dev_setup(&setup) {
            Err(ref e) if e.raw_os_error() == Some(sys::Errno::EINVAL as _) =>
                return self.create_legacy(id, name, ff_effects_max, abs),
            v => v,
        }?;

        for abs in abs {
            self.abs_setup(abs.into())?;
        }

        self.dev_create()
    }

    /// Write input events to the device
    pub fn write(&self, events: &[sys::input_event]) -> io::Result<usize> {
        let events = unsafe { from_raw_parts(events.as_ptr() as *const u8, size_of::<sys::input_event>() * events.len()) };
        nix::unistd::write(self.fd(), events)
            .map(|c| c / size_of::<sys::input_event>()).map_err(convert_error)
    }

    /// Read events from uinput (see `EV_UINPUT`)
    pub fn read(&self, events: &mut [sys::input_event]) -> io::Result<usize> {
        let events = unsafe { from_raw_parts_mut(events.as_mut_ptr() as *mut u8, size_of::<sys::input_event>() * events.len()) };
        nix::unistd::read(self.fd(), events)
            .map(|len| len / size_of::<sys::input_event>()).map_err(convert_error)
    }

    /// Returns the sysfs directory for the input device.
    ///
    /// Note that this path may not exist if sysfs is not mounted in the standard `/sys` location.
    pub fn sys_path(&self) -> io::Result<PathBuf> {
        let sys = self.sys_name()?;
        let sys = CStr::from_bytes_with_nul(&sys).map(|c| c.to_bytes()).unwrap_or(&sys);
        Ok(Path::new("/sys/devices/virtual/input/").join(OsStr::from_bytes(sys)))
    }

    /// The device name of the input device.
    pub fn evdev_name(&self) -> io::Result<OsString> {
        let sys = self.sys_path()?;
        fs::read_dir(&sys)?.filter_map(|e| match e {
            Err(err) => Some(Err(err)),
            Ok(e) => match e.file_type() {
                Err(err) => Some(Err(err)),
                Ok(ty) if ty.is_dir() => {
                    let name = e.file_name();
                    if name.as_bytes().starts_with(b"event") {
                        Some(Ok(e.file_name()))
                    } else {
                        None
                    }
                },
                Ok(..) => None,
            },
        }).next().unwrap_or_else(|| Err(io::Error::new(io::ErrorKind::NotFound, "event input device not found")))
    }

    /// The device node path of the input device.
    ///
    /// Note that this path may not exist if `/dev/input/*` isn't mounted properly.
    pub fn evdev_path(&self) -> io::Result<PathBuf> {
        self.evdev_name().map(|ev| Path::new("/dev/input/").join(ev))
    }

    ioctl_impl! {
        {
            /// `UI_DEV_CREATE`
            @call dev_create = ui_dev_create
        }
        {
            /// `UI_DEV_DESTROY`
            @call dev_destroy = ui_dev_destroy
        }
        {
            /// `UI_DEV_SETUP`
            @set dev_setup(&sys::uinput_setup) = ui_dev_setup
        }
        {
            /// `UI_ABS_SETUP`
            @set abs_setup(&sys::uinput_abs_setup) = ui_abs_setup
        }
        {
            /// `UI_SET_EVBIT`
            @set set_evbit(kinds::EventKind) = ui_set_evbit
        }
        {
            /// `UI_SET_KEYBIT`
            @set set_keybit(::Key) = ui_set_keybit
        }
        {
            /// `UI_SET_RELBIT`
            @set set_relbit(kinds::RelativeAxis) = ui_set_relbit
        }
        {
            /// `UI_SET_ABSBIT`
            @set set_absbit(kinds::AbsoluteAxis) = ui_set_absbit
        }
        {
            /// `UI_SET_MSCBIT`
            @set set_mscbit(kinds::MiscKind) = ui_set_mscbit
        }
        {
            /// `UI_SET_LEDBIT`
            @set set_ledbit(kinds::LedKind) = ui_set_ledbit
        }
        {
            /// `UI_SET_SNDBIT`
            @set set_sndbit(kinds::SoundKind) = ui_set_sndbit
        }
        {
            /// `UI_SET_FFBIT`
            @set set_ffbit(i32) = ui_set_ffbit
        }
        {
            /// `UI_SET_PHYS`
            @set_str set_phys = ui_set_phys
        }
        {
            /// `UI_SET_SWBIT`
            @set set_swbit(kinds::SwitchKind) = ui_set_swbit
        }
        {
            /// `UI_SET_PROPBIT`
            @set set_propbit(kinds::InputProperty) = ui_set_propbit
        }
        {
            /// `UI_BEGIN_FF_UPLOAD`
            @set ff_upload_begin(&mut sys::uinput_ff_upload) = ui_begin_ff_upload
        }
        {
            /// `UI_END_FF_UPLOAD`
            @set ff_upload_end(&sys::uinput_ff_upload) = ui_end_ff_upload
        }
        {
            /// `UI_BEGIN_FF_ERASE`
            @set ff_erase_begin(&mut sys::uinput_ff_erase) = ui_begin_ff_erase
        }
        {
            /// `UI_END_FF_ERASE`
            @set ff_erase_end(&sys::uinput_ff_erase) = ui_end_ff_erase
        }
        {
            /// `UI_GET_SYSNAME`
            @get_str sys_name, sys_name_buf = ui_get_sysname
        }
        {
            /// `UI_GET_VERSION`
            @get version = ui_get_version -> u32
        }
    }
}
