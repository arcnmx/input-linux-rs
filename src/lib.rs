#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/input-linux/0.5.0/")]

//! Userspace bindings to the Linux evdev and uinput subsystems.
//!
//! Start by looking at the [`EvdevHandle`] and [`UInputHandle`] types.

pub use input_linux_sys as sys;
#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize, };

#[macro_use]
mod macros;

mod kinds;
pub use crate::kinds::*;

mod time;
pub use crate::time::EventTime;

mod events;
pub use crate::events::*;

mod keys;
pub use crate::keys::Key;

pub mod evdev;
pub use crate::evdev::EvdevHandle;

pub mod uinput;
pub use crate::uinput::UInputHandle;

pub mod enum_iterator;

pub mod bitmask;
pub use crate::bitmask::Bitmask;

#[cfg(feature = "with-tokio")]
mod tokio;

#[cfg(feature = "with-tokio")]
pub use crate::tokio::EventCodec;

#[repr(C)]
#[derive(Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "with-serde", derive(Deserialize, Serialize))]
/// Identifies an input device.
pub struct InputId {
    /// Identifies the bus where the input device is found, see `sys::BUS_*`
    pub bustype: u16,
    /// The vendor ID of the input device.
    pub vendor: u16,
    /// The product ID of the input device.
    pub product: u16,
    /// The version of the input device.
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
        *<&InputId>::from(&id)
    }
}

impl From<InputId> for sys::input_id {
    fn from(id: InputId) -> Self {
        *<&sys::input_id>::from(&id)
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "with-serde", derive(Deserialize, Serialize))]
/// A descriptor used to create a virtual uinput absolute axis.
pub struct AbsoluteInfoSetup {
    /// The axis to attach the `AbsoluteInfo` constraints to.
    pub axis: AbsoluteAxis,
    /// Describes the capabilities of the absolute axis.
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
#[derive(Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "with-serde", derive(Deserialize, Serialize))]
/// Describes the capabilities and constraints of an input device's absolute axis.
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
        *<&AbsoluteInfo>::from(&info)
    }
}

impl From<AbsoluteInfo> for sys::input_absinfo {
    fn from(info: AbsoluteInfo) -> Self {
        *<&sys::input_absinfo>::from(&info)
    }
}
