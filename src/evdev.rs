use std::io;
use std::mem::uninitialized;
use std::os::unix::io::{RawFd, AsRawFd};
use sys;
use ::{InputId};

pub use sys::EV_VERSION;

/// A handle to an input device allowing the use of ioctls
///
/// Ownership of the file descriptor is not transferred, and it must stay open
/// for this object's lifetime. It will not be closed automatically.
pub struct EvdevHandle(RawFd);

const STRING_BUFFER_LENGTH: usize = 0x200;

fn convert_error(e: sys::Error) -> io::Error {
    match e {
        sys::Error::Sys(errno) => errno.into(),
        _ => sys::Errno::EINVAL.into(),
    }
}

macro_rules! evdev_impl {
    ($(#[$attr:meta])* @get $f:ident = $ev:ident -> $ret:ty) => {
        $(#[$attr])*
        pub fn $f(&self) -> io::Result<$ret> {
            unsafe {
                let mut v = uninitialized();
                sys::$ev(self.0, &mut v)
                    .map(|_| v.into())
                    .map_err(convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @get_buf $f:ident = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self, buffer: &mut [u8]) -> io::Result<usize> {
            unsafe {
                sys::$ev(self.0, buffer)
                    .map(|len| len as _)
                    .map_err(convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @get_str $f:ident, $fbuf:ident = $ev:ident) => {
        evdev_impl! {
            $(#[$attr])*
            @get_buf $fbuf = $ev
        }

        $(#[$attr])*
        pub fn $f(&self) -> io::Result<Vec<u8>> {
            let mut buf = vec![0; STRING_BUFFER_LENGTH];
            self.$fbuf(&mut buf[..]).map(move |len| {
                buf.truncate(len as _);
                buf
            })
        }
    };
    ($(#[$attr:meta])* @set $f:ident($in:ty) = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self, value: $in) -> io::Result<()> {
            unsafe {
                sys::$ev(self.0, value as _)
                    .map(drop)
                    .map_err(convert_error)
            }
        }
    };
    ($({ $($tt:tt)* })*) => {
        $(
            evdev_impl! {$($tt)*}
        )*
    };
}

impl EvdevHandle {
    /// Create a new handle using an existing open file object.
    pub fn new<F: AsRawFd>(fd: &F) -> Self {
        EvdevHandle(fd.as_raw_fd())
    }

    /// Create a new handle from a raw file descriptor.
    pub fn from_fd(fd: RawFd) -> Self {
        EvdevHandle(fd)
    }

    evdev_impl! {
        {
            /// `EVIOCGVERSION`
            @get driver_version = ev_get_version -> i32
        }
        {
            /// `EVIOCGID`
            @get device_id = ev_get_id -> InputId
        }
        {
            /// `EVIOGREP`
            @get repeat_settings = ev_get_rep -> [u32; 2]
        }
        {
            /// `EVIOSREP`
            @set set_repeat_settings(&[u32; 2]) = ev_set_rep
        }
        {
            /// `EVIOCGKEYCODE`
            @get keycode_legacy = ev_get_keycode -> [u32; 2]
        }
        {
            /// `EVIOCGKEYCODE`_V2
            @get keycode = ev_get_keycode_v2 -> sys::input_keymap_entry
        }
        {
            /// `EVIOCSKEYCODE`
            @set set_keycode_legacy(&[u32; 2]) = ev_set_keycode
        }
        {
            /// `EVIOCSKEYCODE`_V2
            @set set_keycode(&sys::input_keymap_entry) = ev_set_keycode_v2
        }
        {
            /// `EVIOCGNAME`
            @get_str device_name, device_name_buf = ev_get_name
        }
        {
            /// `EVIOCGPHYS`
            @get_str physical_location, physical_location_buf = ev_get_phys
        }
        {
            /// `EVIOCGUNIQ`
            @get_str unique_id, unique_id_buf = ev_get_uniq
        }
        {
            /// `EVIOCGPROP`
            @get_buf device_properties = ev_get_prop
        }
        {
            /// `EVIOCGMTSLOTS`
            @get_buf multi_touch_slots = ev_get_mtslots
        }
        {
            /// `EVIOCGKEY`
            @get_buf key_state = ev_get_key
        }
        {
            /// `EVIOCGLED`
            @get_buf led_state = ev_get_led
        }
        {
            /// `EVIOCGSND`
            @get_buf sounds_state = ev_get_snd
        }
        {
            /// `EVIOCGSW`
            @get_buf switch_state = ev_get_sw
        }
        {
            /// `EVIOCSFF`
            @set send_force_feedback(&mut sys::ff_effect) = ev_send_ff
        }
        {
            /// `EVIOCRMFF`
            @set erase_force_feedback(i16) = ev_erase_ff
        }
        {
            /// `EVIOCGEFFECTS`
            @get effects_count = ev_get_effects -> i32
        }
        {
            /// `EVIOCGMASK`
            @get event_mask = ev_get_mask -> sys::input_mask
        }
        {
            /// `EVIOCSMASK`
            @set set_event_mask(&sys::input_mask) = ev_set_mask
        }
    }

    /// `EVIOCGBIT`
    pub fn event_bits(&self, ev: usize, buffer: &mut [u8]) -> io::Result<()> {
        unsafe {
            sys::ev_get_bit(self.0, ev as _, buffer)
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCGABS`
    pub fn abs_info(&self, abs: usize) -> io::Result<sys::input_absinfo> {
        unsafe {
            let mut info = uninitialized();
            sys::ev_get_abs(self.0, abs as _, &mut info)
                .map(|_| info)
                .map_err(convert_error)
        }
    }

    /// `EVIOCSABS`
    pub fn set_abs_info(&self, abs: usize, info: &sys::input_absinfo) -> io::Result<()> {
        unsafe {
            sys::ev_set_abs(self.0, abs as _, info)
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCGRAB`
    pub fn grab(&self, grab: bool) -> io::Result<()> {
        unsafe {
            sys::ev_grab(self.0, if grab { 1 } else { 0 })
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCREVOKE`
    pub fn revoke(&self) -> io::Result<()> {
        unsafe {
            sys::ev_revoke(self.0, 0)
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCSCLOCKID`
    pub fn set_clock_id(&self, value: i32) -> io::Result<()> {
        unsafe {
            sys::ev_set_clockid(self.0, &value)
                .map(drop)
                .map_err(convert_error)
        }
    }
}
