use std::{io, ptr};
use std::os::unix::io::{RawFd, AsRawFd};
use std::os::raw::c_char;
use std::mem::{uninitialized, size_of};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::ffi::CStr;
use sys;
use nix;
use ::{InputId, AbsoluteInfoSetup, kinds};
use macros::convert_error;

pub use sys::UINPUT_VERSION;
pub use sys::UINPUT_MAX_NAME_SIZE;

/// A handle to a uinput allowing the use of ioctls
///
/// Ownership of the file descriptor is not transferred, and it must stay open
/// for this object's lifetime. It will not be closed automatically.
pub struct UInputHandle(RawFd);

fn copy_name(dest: &mut [c_char; UINPUT_MAX_NAME_SIZE as usize], name: &CStr) -> io::Result<()> {
    let name = name.to_bytes_with_nul();
    if name.len() > UINPUT_MAX_NAME_SIZE as usize {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "name too long"))
    } else {
        unsafe {
            ptr::copy_nonoverlapping(name.as_ptr(), dest.as_mut_ptr() as *mut _, name.len());
        }

        Ok(())
    }
}

impl UInputHandle {
    /// Create a new handle using an existing open file object.
    pub fn new<F: AsRawFd>(fd: &F) -> Self {
        UInputHandle(fd.as_raw_fd())
    }

    /// Create a new handle from a raw file descriptor.
    pub fn from_fd(fd: RawFd) -> Self {
        UInputHandle(fd)
    }

    /// Create a new uinput device using the legacy `UI_DEV_CREATE` interface
    pub fn create_legacy(&self, id: &InputId, name: &CStr, ff_effects_max: u32, abs: &[AbsoluteInfoSetup]) -> io::Result<()> {
        self.dev_create()?;

        // race condition here?

        let mut setup: sys::uinput_user_dev = unsafe { uninitialized() };
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
        nix::unistd::write(self.0, setup)
            .map(drop).map_err(convert_error)
    }

    /// Create a new uinput device, and fall back on the legacy interface if necessary
    pub fn create(&self, id: &InputId, name: &CStr, ff_effects_max: u32, abs: &[AbsoluteInfoSetup]) -> io::Result<()> {
        let mut setup: sys::uinput_setup = unsafe { uninitialized() };
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

        Ok(())
    }

    /// Write an input event to the device
    pub fn write(&self, events: &[sys::input_event]) -> io::Result<()> {
        let events = unsafe { from_raw_parts(events.as_ptr() as *const u8, size_of::<sys::input_event>() * events.len()) };
        nix::unistd::write(self.0, events)
            .map(drop).map_err(convert_error)
    }

    /// Read an event from uinput (see `EV_UINPUT`)
    pub fn read(&self, events: &mut [sys::input_event]) -> io::Result<usize> {
        let events = unsafe { from_raw_parts_mut(events.as_mut_ptr() as *mut u8, size_of::<sys::input_event>() * events.len()) };
        nix::unistd::read(self.0, events)
            .map(|len| len / size_of::<sys::input_event>()).map_err(convert_error)
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
            @get_str sysname, sysname_buf = ui_get_sysname
        }
        {
            /// `UI_GET_VERSION`
            @get version = ui_get_version -> u32
        }
    }
}
