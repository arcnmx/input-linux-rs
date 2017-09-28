use std::io;
use sys;

pub const STRING_BUFFER_LENGTH: usize = 0x200;

pub fn convert_error(e: sys::Error) -> io::Error {
    match e {
        sys::Error::Sys(errno) => errno.into(),
        _ => sys::Errno::EINVAL.into(),
    }
}

macro_rules! ioctl_impl {
    ($(#[$attr:meta])* @get $f:ident = $ev:ident -> $ret:ty) => {
        $(#[$attr])*
        pub fn $f(&self) -> io::Result<$ret> {
            unsafe {
                let mut v = ::std::mem::uninitialized();
                sys::$ev(self.0, &mut v)
                    .map(|_| v.into())
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @call $f:ident = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self) -> io::Result<()> {
            unsafe {
                sys::$ev(self.0)
                    .map(drop)
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @get_buf $f:ident($ty:ty) = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self, buffer: &mut [$ty]) -> io::Result<usize> {
            unsafe {
                sys::$ev(self.0, &mut *(buffer as *mut [$ty] as *mut [_]))
                    .map(|len| len as _)
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @get_str $f:ident, $fbuf:ident = $ev:ident) => {
        ioctl_impl! {
            $(#[$attr])*
            @get_buf $fbuf(u8) = $ev
        }

        $(#[$attr])*
        pub fn $f(&self) -> io::Result<Vec<u8>> {
            let mut buf = vec![0; ::macros::STRING_BUFFER_LENGTH];
            self.$fbuf(&mut buf[..]).map(move |len| {
                buf.truncate(len as _);
                buf
            })
        }
    };
    ($(#[$attr:meta])* @set_str $f:ident = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self, value: &::std::ffi::CStr) -> io::Result<()> {
            unsafe {
                sys::$ev(self.0, value.as_ptr())
                    .map(drop)
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @set $f:ident($in:ty) = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self, value: $in) -> io::Result<()> {
            unsafe {
                sys::$ev(self.0, value as _)
                    .map(drop)
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($({ $($tt:tt)* })*) => {
        $(
            ioctl_impl! {$($tt)*}
        )*
    };
}
