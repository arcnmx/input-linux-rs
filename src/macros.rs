use std::io;
use sys;

pub(crate) const STRING_BUFFER_LENGTH: usize = 0x200;

pub(crate) fn convert_error(e: sys::Error) -> io::Error {
    match e {
        sys::Error::Sys(errno) => errno.into(),
        _ => sys::Errno::EINVAL.into(),
    }
}

macro_rules! impl_iterable {
    (@impliter $name: ident($start:expr, $count:expr)) => {
        impl ::bitmask::BitmaskTrait for $name {
            type Array = [u8; (Self::COUNT + 7) / 8];
            type Index = $name;

            fn array_default() -> Self::Array { unsafe { ::std::mem::zeroed() } }
            fn array_slice(array: &Self::Array) -> &[u8] { array }
            fn array_slice_mut(array: &mut Self::Array) -> &mut [u8] { array }
            fn index(index: Self::Index) -> usize { index as usize }
        }

        impl ::enum_iterator::IterableEnum for $name {
            fn iter_next(v: usize) -> Option<(usize, Self)> {
                if v < Self::COUNT {
                    unsafe {
                        Some((v + 1, ::std::mem::transmute(v as u16)))
                    }
                } else {
                    None
                }
            }

            fn iter() -> ::enum_iterator::EnumIterator<Self> {
                ::enum_iterator::EnumIterator::new($start)
            }
        }

        impl $name {
            /// The maximum valid value.
            pub const COUNT: usize = $count as usize;

            /// An iterator over all values of the enum.
            pub fn iter() -> ::enum_iterator::EnumIterator<Self> {
                ::enum_iterator::IterableEnum::iter()
            }

            /// A bitmask that can contain all values of the enum.
            pub fn bitmask() -> ::bitmask::Bitmask<Self> {
                Default::default()
            }
        }

        impl From<$name> for u16 {
            fn from(v: $name) -> Self {
                v as _
            }
        }
    };
    (@implcode $name: ident($start:expr, $count:expr)) => {
        impl $name {
            /// Instantiates the enum from a raw code value.
            pub fn from_code(code: u16) -> Result<Self, ::kinds::RangeError> {
                use std::mem;

                if code < Self::COUNT as u16 {
                    Ok(unsafe { mem::transmute(code) })
                } else {
                    Err(Default::default())
                }
            }
        }

        impl std::convert::TryFrom<u16> for $name {
            type Error = ::kinds::RangeError;

            fn try_from(code: u16) -> Result<Self, Self::Error> {
                Self::from_code(code)
            }
        }
    };
    ($name: ident($start:expr, $count:expr)) => {
        impl_iterable! { @impliter $name($start, $count) }
        impl_iterable! { @implcode $name($start, $count) }
    };
    (@nofromcode $name: ident($start:expr, $count:expr)) => {
        impl_iterable! { @impliter $name($start, $count) }
    };
}

macro_rules! ioctl_impl {
    ($(#[$attr:meta])* @get $f:ident = $ev:ident -> $ret:ty) => {
        $(#[$attr])*
        pub fn $f(&self) -> io::Result<$ret> {
            unsafe {
                let mut v = ::std::mem::MaybeUninit::uninit();
                sys::$ev(self.fd(), &mut *v.as_mut_ptr())
                    .map(|_| v.assume_init().into())
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @call $f:ident = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self) -> io::Result<()> {
            unsafe {
                sys::$ev(self.fd())
                    .map(drop)
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @get_buf $f:ident($ty:ty) = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self, buffer: &mut [$ty]) -> io::Result<usize> {
            unsafe {
                sys::$ev(self.fd(), &mut *(buffer as *mut [$ty] as *mut [_]))
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
                sys::$ev(self.fd(), value.as_ptr())
                    .map(drop)
                    .map_err(::macros::convert_error)
            }
        }
    };
    ($(#[$attr:meta])* @set $f:ident($in:ty) = $ev:ident) => {
        $(#[$attr])*
        pub fn $f(&self, value: $in) -> io::Result<()> {
            unsafe {
                sys::$ev(self.fd(), value as _)
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

macro_rules! impl_bitmasks {
    { $kind:path, $event: expr, $name_mask:ident, $name_mask_set:ident, $name_bits:ident } => {
        /// `EVIOCGMASK`
        pub fn $name_mask(&self) -> io::Result<::bitmask::Bitmask<$kind>> {
            let mut bitmask = ::bitmask::Bitmask::default();
            self.event_mask_raw($event, &mut bitmask).map(|_| bitmask)
        }

        /// `EVIOCSMASK`
        pub fn $name_mask_set(&self, bitmask: &::bitmask::Bitmask<$kind>) -> io::Result<()> {
            self.set_event_mask_raw($event, bitmask)
        }

        /// `EVIOCGBIT`
        pub fn $name_bits(&self) -> io::Result<::bitmask::Bitmask<$kind>> {
            let mut bitmask = ::bitmask::Bitmask::default();
            self.event_bits_raw($event, &mut bitmask).map(|_| bitmask)
        }
    };
}
