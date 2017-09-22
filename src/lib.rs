//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/input-linux-rs/")]

extern crate input_linux_sys as sys;

mod kinds;
pub use kinds::*;

mod time;
pub use time::EventTime;

mod events;
pub use events::*;

mod keys;
pub use keys::Key;

pub mod evdev;

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct InputId {
    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

impl<'a> From<&'a sys::input_id> for &'a InputId {
    fn from(id: &'a sys::input_id) -> Self {
        let raw = id as *const _ as *const _;
        unsafe { &*raw }
    }
}

impl<'a> From<&'a InputId> for &'a sys::input_id {
    fn from(id: &'a InputId) -> Self {
        let raw = id as *const _ as *const _;
        unsafe { &*raw }
    }
}

impl From<sys::input_id> for InputId {
    fn from(id: sys::input_id) -> Self {
        <&InputId>::from(&id).clone()
    }
}

impl From<InputId> for sys::input_id {
    fn from(id: InputId) -> Self {
        <&sys::input_id>::from(&id).clone()
    }
}
