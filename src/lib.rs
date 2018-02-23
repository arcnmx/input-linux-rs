//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/input-linux-rs/")]

pub extern crate input_linux_sys as sys;
extern crate nix;

#[macro_use]
mod macros;

mod kinds;
pub use kinds::*;

mod time;
pub use time::EventTime;

mod events;
pub use events::*;

mod keys;
pub use keys::Key;

pub mod evdev;
pub use evdev::EvdevHandle;

pub mod uinput;
pub use uinput::UInputHandle;

#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "tokio")]
pub use tokio::EventDecoder;

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

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct AbsoluteInfoSetup {
    pub axis: AbsoluteAxis,
    pub info: AbsoluteInfo,
}

impl<'a> From<&'a AbsoluteInfoSetup> for &'a sys::uinput_abs_setup {
    fn from(info: &'a AbsoluteInfoSetup) -> Self {
        let raw = info as *const _ as *const _;
        unsafe { &*raw }
    }
}

impl From<AbsoluteInfoSetup> for sys::uinput_abs_setup {
    fn from(info: AbsoluteInfoSetup) -> Self {
        use std::mem;
        unsafe {
            mem::transmute(info)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct AbsoluteInfo {
    /// latest reported value for the axis
    pub value: i32,
    /// specifies minimum value for the axis
    pub minimum: i32,
    /// specifies maximum value for the axis
    pub maximum: i32,
    /// specifies fuzz value that is used to filter noise from the event stream
    pub fuzz: i32,
    /// values that are within this value will be discarded by joydev interface and reported as 0 instead
    pub flat: i32,
    /// specifies resolution for the values reported for the axis
    ///
    /// Resolution for main axes (ABS_X, ABS_Y, ABS_Z) is reported in units per
    /// millimeter (units/mm), resolution for rotational axes (ABS_RX, ABS_RY,
    /// ABS_RZ) is reported in units per radian.
    pub resolution: i32,
}

impl<'a> From<&'a sys::input_absinfo> for &'a AbsoluteInfo {
    fn from(info: &'a sys::input_absinfo) -> Self {
        let raw = info as *const _ as *const _;
        unsafe { &*raw }
    }
}

impl<'a> From<&'a AbsoluteInfo> for &'a sys::input_absinfo {
    fn from(info: &'a AbsoluteInfo) -> Self {
        let raw = info as *const _ as *const _;
        unsafe { &*raw }
    }
}

impl From<sys::input_absinfo> for AbsoluteInfo {
    fn from(info: sys::input_absinfo) -> Self {
        <&AbsoluteInfo>::from(&info).clone()
    }
}

impl From<AbsoluteInfo> for sys::input_absinfo {
    fn from(info: AbsoluteInfo) -> Self {
        <&sys::input_absinfo>::from(&info).clone()
    }
}
