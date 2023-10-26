//! An interface to the Linux kernel's event devices (`/dev/input/*`).

use std::io;
use std::mem::{MaybeUninit, size_of};
use std::slice::from_raw_parts_mut;
use std::os::unix::io::{RawFd, AsRawFd, IntoRawFd, FromRawFd};
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};
use nix;
use crate::sys;
use crate::{
    AbsoluteAxis, AbsoluteInfo, AutorepeatKind, EventKind, InputId,
    InputProperty, Key, LedKind, MiscKind, RelativeAxis, SoundKind, SwitchKind,
    ForceFeedbackKind, ForceFeedbackStatusKind,
    InputEvent,
};
use crate::macros::convert_error;
use crate::bitmask::Bitmask;

pub use sys::EV_VERSION;

/// A handle to an input device allowing the use of ioctls
pub struct EvdevHandle<F>(F);

impl<F> EvdevHandle<F> {
    /// Create a new handle using an existing open file object.
    pub const fn new(fd: F) -> Self {
        EvdevHandle(fd)
    }

    /// Extracts the contained handle.
    pub fn into_inner(self) -> F {
        self.0
    }

    /// A reference to the contained handle.
    pub const fn as_inner(&self) -> &F {
        &self.0
    }

    /// A mutable reference to the contained handle.
    pub fn as_inner_mut(&mut self) -> &mut F {
        &mut self.0
    }
}

impl<F: AsRawFd> AsFd for EvdevHandle<F> {
    fn as_fd<'a>(&'a self) -> BorrowedFd<'a> {
        unsafe {
            BorrowedFd::borrow_raw(self.fd())
        }
    }
}

impl<F: AsRawFd> AsRawFd for EvdevHandle<F> {
    fn as_raw_fd(&self) -> RawFd {
        self.fd()
    }
}

impl<F: IntoRawFd> IntoRawFd for EvdevHandle<F> {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl<F: FromRawFd> FromRawFd for EvdevHandle<F> {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        EvdevHandle(FromRawFd::from_raw_fd(fd))
    }
}

impl EvdevHandle<OwnedFd> {
    /// Create a new handle from a raw file descriptor.
    pub unsafe fn from_fd(fd: RawFd) -> Self {
        FromRawFd::from_raw_fd(fd)
    }
}

impl<F: AsRawFd> EvdevHandle<F> {
    #[inline]
    fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    /// Read events from the input device
    pub fn read(&self, events: &mut [sys::input_event]) -> io::Result<usize> {
        let events = unsafe { from_raw_parts_mut(events.as_mut_ptr() as *mut u8, size_of::<sys::input_event>() * events.len()) };
        nix::unistd::read(self.fd(), events)
            .map(|len| len / size_of::<sys::input_event>())
            .map_err(convert_error)
    }

    /// Read events from the input device
    pub fn read_input_events<'e>(&self, events: &'e mut [MaybeUninit<InputEvent>]) -> io::Result<&'e mut [InputEvent]> {
        let res = {
            let events = unsafe { from_raw_parts_mut(events.as_mut_ptr() as *mut sys::input_event, events.len()) };
            self.read(events)
        };
        res.and_then(|count| {
            let events = &mut events[..count];
            for event in events.iter() {
                let event = event.as_ptr() as *const sys::input_event;
                let _ = InputEvent::from_raw(unsafe { &*event })?;
            }
            Ok(unsafe {
                from_raw_parts_mut(events.as_mut_ptr() as *mut InputEvent, events.len())
            })
        })
    }

    /// Read a single event from the input device
    pub fn read_input_event(&self) -> io::Result<InputEvent> {
        let mut events = [MaybeUninit::<InputEvent>::uninit()];
        let res = self.read_input_events(&mut events)
            .and_then(|read| match read.is_empty() {
                true => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "empty evdev read")),
                false => Ok(()),
            });
        res.map(|()| unsafe {
            events[0].assume_init()
        })
    }

    /// Read a single event from the input device
    pub fn read_event(&self) -> io::Result<crate::Event> {
        self.read_input_event()
            .map(From::from)
    }

    /// Write events to the input device
    pub fn write(&self, events: &[sys::input_event]) -> io::Result<usize> {
        let events = unsafe { from_raw_parts_mut(events.as_ptr() as *mut u8, size_of::<sys::input_event>() * events.len()) };
        nix::unistd::write(self, events)
            .map(|len| len / size_of::<sys::input_event>()).map_err(convert_error)
    }

    ioctl_impl! {
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
            @get repeat_settings = ev_get_rep -> sys::repeat_settings
        }
        {
            /// `EVIOSREP`
            @set set_repeat_settings(&sys::repeat_settings) = ev_set_rep
        }
        {
            /// `EVIOCGKEYCODE`
            @get keycode_legacy = ev_get_keycode -> [u32; 2]
        }
        {
            /// `EVIOCGKEYCODE_V2`
            @get keycode = ev_get_keycode_v2 -> sys::input_keymap_entry
        }
        {
            /// `EVIOCSKEYCODE`
            @set set_keycode_legacy(&[u32; 2]) = ev_set_keycode
        }
        {
            /// `EVIOCSKEYCODE_V2`
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
            @get_buf device_properties_raw(u8) = ev_get_prop
        }
        {
            /// `EVIOCGKEY`
            @get_buf key_state(u8) = ev_get_key
        }
        {
            /// `EVIOCGLED`
            @get_buf led_state(u8) = ev_get_led
        }
        {
            /// `EVIOCGSND`
            @get_buf sounds_state(u8) = ev_get_snd
        }
        {
            /// `EVIOCGSW`
            @get_buf switch_state(u8) = ev_get_sw
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
    }

    /// `EVIOCGPROP`
    pub fn device_properties(&self) -> io::Result<Bitmask<InputProperty>> {
        let mut bitmask = Bitmask::default();
        self.device_properties_raw(&mut bitmask).map(|_| bitmask)
    }

    /// `EVIOCGMTSLOTS`
    ///
    /// Warning: the current implementation can leak uninitialized heap memory into `values`
    pub fn multi_touch_slots(&self, code: AbsoluteAxis, values: &mut [i32]) -> io::Result<()> {
        let input_len = values.len() + 1;
        let mut buf = Vec::<i32>::with_capacity(input_len);

        // a perfect example of how not to use ev_get_mtslots
        unsafe {
            buf.set_len(input_len);
            buf[0] = code as _;

            // the first field isn't counted in the len of the fat pointer
            let ptr = from_raw_parts_mut(buf.as_mut_ptr(), values.len()) as *mut _;
            sys::ev_get_mtslots(self.fd(), ptr as *mut sys::input_mt_request_layout<[i32]>)
                .map_err(convert_error)?;
        }

        values.copy_from_slice(&buf[1..]);
        Ok(())
    }

    impl_bitmasks! { EventKind, EventKind::Synchronize,
        event_mask_events, set_event_mask_events,
        event_bits
    }

    impl_bitmasks! { Key, EventKind::Key,
        key_mask, set_key_mask,
        key_bits
    }

    impl_bitmasks! { RelativeAxis, EventKind::Relative,
        relative_mask, set_relative_mask,
        relative_bits
    }

    impl_bitmasks! { AbsoluteAxis, EventKind::Absolute,
        absolute_mask, set_absolute_mask,
        absolute_bits
    }

    impl_bitmasks! { MiscKind, EventKind::Misc,
        misc_mask, set_misc_mask,
        misc_bits
    }

    impl_bitmasks! { SwitchKind, EventKind::Switch,
        switch_mask, set_switch_mask,
        switch_bits
    }

    impl_bitmasks! { LedKind, EventKind::Led,
        led_mask, set_led_mask,
        led_bits
    }

    impl_bitmasks! { SoundKind, EventKind::Sound,
        sound_mask, set_sound_mask,
        sound_bits
    }

    impl_bitmasks! { AutorepeatKind, EventKind::Autorepeat,
        autorepeat_mask, set_autorepeat_mask,
        autorepeat_bits
    }

    impl_bitmasks! { ForceFeedbackKind, EventKind::ForceFeedback,
        force_feedback_mask, set_force_feedback_mask,
        force_feedback_bits
    }

    impl_bitmasks! { ForceFeedbackStatusKind, EventKind::ForceFeedbackStatus,
        force_feedback_status_mask, set_force_feedback_status_mask,
        force_feedback_status_bits
    }

    /// `EVIOCGMASK`
    pub fn event_mask_raw(&self, kind: EventKind, buffer: &mut [u8]) -> io::Result<()> {
        unsafe {
            let mut mask = sys::input_mask {
                type_: kind as _,
                codes_size: buffer.len() as _,
                codes_ptr: buffer.as_mut_ptr() as usize as _,
            };

            sys::ev_get_mask(self.fd(), &mut mask)
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCSMASK`
    pub fn set_event_mask_raw(&self, kind: EventKind, buffer: &[u8]) -> io::Result<()> {
        unsafe {
            let mask = sys::input_mask {
                type_: kind as _,
                codes_size: buffer.len() as _,
                codes_ptr: buffer.as_ptr() as usize as _,
            };

            sys::ev_set_mask(self.fd(), &mask)
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCGBIT`
    pub fn event_bits_raw(&self, kind: EventKind, buffer: &mut [u8]) -> io::Result<usize> {
        unsafe {
            sys::ev_get_bit(self.fd(), kind as _, buffer)
                .map(|i| i as _)
                .map_err(convert_error)
        }
    }

    /// `EVIOCGABS`
    pub fn absolute_info(&self, abs: AbsoluteAxis) -> io::Result<AbsoluteInfo> {
        unsafe {
            let mut info = MaybeUninit::uninit();
            sys::ev_get_abs(self.fd(), abs as _, &mut *info.as_mut_ptr())
                .map(|_| info.assume_init().into())
                .map_err(convert_error)
        }
    }

    /// `EVIOCSABS`
    pub fn set_absolute_info(&self, abs: AbsoluteAxis, info: &AbsoluteInfo) -> io::Result<()> {
        unsafe {
            let info: &sys::input_absinfo = info.into();
            sys::ev_set_abs(self.fd(), abs as _, info)
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCGRAB`
    pub fn grab(&self, grab: bool) -> io::Result<()> {
        unsafe {
            sys::ev_grab(self.fd(), if grab { 1 } else { 0 })
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCREVOKE`
    pub fn revoke(&self) -> io::Result<()> {
        unsafe {
            sys::ev_revoke(self.fd(), 0)
                .map(drop)
                .map_err(convert_error)
        }
    }

    /// `EVIOCSCLOCKID`
    pub fn set_clock_id(&self, value: i32) -> io::Result<()> {
        unsafe {
            sys::ev_set_clockid(self.fd(), &value)
                .map(drop)
                .map_err(convert_error)
        }
    }
}
