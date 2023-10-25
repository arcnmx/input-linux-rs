use core::convert::TryFrom;
use core::mem::{transmute, size_of};
use core::hint::unreachable_unchecked;
use crate::sys::input_event;
use crate::{
    EventTime, RangeError, KeyState,
    EventKind, SynchronizeKind, Key, RelativeAxis, AbsoluteAxis,
    SwitchKind, MiscKind, LedKind, AutorepeatKind, SoundKind, UInputKind,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// Synchronization events are used by evdev to group events or convey other
/// out-of-band information.
pub struct SynchronizeEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The type of synchronization event.
    pub kind: SynchronizeKind,
    /// An associated value with the event.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// An event that indicates the state of a key has changed.
pub struct KeyEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The key that triggered the event.
    pub key: Key,
    /// The value of the event.
    pub value: KeyState,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// Events that occur when the state of a relative axis is changed.
pub struct RelativeEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The axis associated with the event.
    pub axis: RelativeAxis,
    /// The relative distance of the axis event.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// Events that occur when the state of an absolute axis is changed.
pub struct AbsoluteEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The axis associated with the event.
    pub axis: AbsoluteAxis,
    /// The absolute value of the axis.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// Special switch events.
pub struct SwitchEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The switch that triggered the event.
    pub switch: SwitchKind,
    /// The state of the switch.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// Miscellaneous events.
pub struct MiscEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The kind of miscellaneous event.
    pub kind: MiscKind,
    /// The state/value of the event.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// An event that indicates whether the specified LED should turn on/off.
pub struct LedEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The led associated with the event.
    pub led: LedKind,
    /// The state of the led.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// An event that configures the autorepeat behaviour of the input device.
pub struct AutorepeatEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The kind of autorepeat event.
    pub kind: AutorepeatKind,
    /// The value of the event.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// An event that indicates the device should play a specified sound.
pub struct SoundEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The sound to play.
    pub sound: SoundKind,
    /// The value or state associated with the sound.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// A special event type used to send force-feedback events to uinput.
pub struct UInputEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    event: EventKind,
    /// The code of the event.
    pub code: UInputKind,
    /// The unique request ID.
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// A generic event.
pub struct InputEvent {
    /// The timestamp associated with the event.
    pub time: EventTime,
    /// The type of event that occurred.
    pub kind: EventKind,
    /// The code of the event.
    ///
    /// The meaning of this code depends on the `kind` of event. Using the typed
    /// events via [`Event`] and [`EventRef`] is recommended.
    pub code: u16,
    /// The value of the event.
    ///
    /// The interpretation of this value depends on the `kind` of event.
    pub value: i32,
}

impl InputEvent {
    /// Constructs an empty event.
    pub const fn zeroed() -> Self {
        Self {
            time: EventTime::new(0, 0),
            kind: EventKind::Synchronize,
            code: SynchronizeKind::Report.code(),
            value: 0,
        }
    }

    /// Reinterprets a raw [`input_event`].
    pub fn with_raw(event: input_event) -> Result<Self, RangeError> {
        EventKind::from_type(event.type_).map(move |_| unsafe {
            Self::with_raw_unchecked(event)
        })
    }

    /// Reinterprets a raw [`input_event`].
    ///
    /// # Safety
    ///
    /// The input event must have a valid [event kind](EventKind).
    pub unsafe fn with_raw_unchecked(event: input_event) -> Self {
        unsafe {
            transmute(event)
        }
    }

    /// Reinterpret this event as an array of bytes.
    pub fn into_bytes(self) -> [u8; size_of::<InputEvent>()] {
        unsafe {
            transmute(self)
        }
    }

    /// Reinterpret this event as a byte slice.
    pub fn as_bytes(&self) -> &[u8; size_of::<InputEvent>()] {
        unsafe {
            transmute(self)
        }
    }

    /// Reinterpret this event as a mutable byte slice.
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8; size_of::<InputEvent>()] {
        transmute(self)
    }
}

#[test]
fn input_event_zeroed() {
    use core::mem::MaybeUninit;

    let event = InputEvent::zeroed().into_raw();
    let zeroed: input_event = unsafe { MaybeUninit::zeroed().assume_init() };
    assert_eq!(event, zeroed);
}

impl SynchronizeEvent {
    /// Synchronization event
    pub const fn report(time: EventTime) -> Self {
        Self::new(time, SynchronizeKind::Report, 0)
    }
}

macro_rules! event_impl {
    (@impl_event InputEvent $($tt:tt)*) => { };
    (@impl_event $name:ident $code:ident $kind:path, $codekind:ident, $valuekind:ident) => {
        impl GenericEvent for $name {
            fn event_kind(&self) -> EventKind { $kind }
            fn time(&self) -> &EventTime { &self.time }
            fn code(&self) -> u16 { self.$code as _ }
            fn value(&self) -> i32 { self.value.into() }

            fn with_event(event: InputEvent) -> Result<Self, RangeError> {
                $codekind::from_code(event.code)
                    .and_then(move |_| if event.kind == $kind {
                        Ok(unsafe { Self::with_event_unchecked(event) })
                    } else {
                        Err(Default::default())
                    })
            }

            unsafe fn with_event_unchecked(event: InputEvent) -> Self {
                transmute(event)
            }

            fn from_ref(event: &InputEvent) -> Result<&Self, RangeError> {
                $codekind::from_code(event.code)
                    .and_then(move |_| if event.kind == $kind {
                        Ok(unsafe { Self::from_event(event) })
                    } else {
                        Err(Default::default())
                    })
            }

            fn from_mut(event: &mut InputEvent) -> Result<&mut Self, RangeError> {
                $codekind::from_code(event.code)
                    .and_then(move |_| if event.kind == $kind {
                        Ok(unsafe { Self::from_event_mut(event) })
                    } else {
                        Err(Default::default())
                    })
            }
        }

        impl<'a> TryFrom<&'a InputEvent> for &'a $name {
            type Error = RangeError;

            fn try_from(event: &'a InputEvent) -> Result<Self, Self::Error> {
                $name::from_ref(event)
            }
        }

        impl TryFrom<InputEvent> for $name {
            type Error = RangeError;

            fn try_from(event: InputEvent) -> Result<Self, Self::Error> {
                $name::from_ref(&event).map(|&e| e)
            }
        }

        impl<'a> TryFrom<&'a mut InputEvent> for &'a mut $name {
            type Error = RangeError;

            fn try_from(event: &'a mut InputEvent) -> Result<Self, Self::Error> {
                $name::from_mut(event)
            }
        }

        impl<'a> From<&'a $name> for &'a InputEvent {
            fn from(event: &'a $name) -> Self {
                event.as_event()
            }
        }

        impl<'a> From<&'a $name> for InputEvent {
            fn from(event: &'a $name) -> Self {
                event.as_event().clone()
            }
        }

        impl From<$name> for InputEvent {
            fn from(event: $name) -> Self {
                From::from(&event)
            }
        }

        impl AsRef<InputEvent> for $name {
            fn as_ref(&self) -> &InputEvent {
                self.as_event()
            }
        }

        impl<'a> $name {
            /// Creates a new event from the given code and value.
            pub const fn new(time: EventTime, $code: $codekind, value: $valuekind) -> Self {
                $name {
                    time,
                    event: $kind,
                    $code: $code,
                    value,
                }
            }

            /// Reinterpret a generic event without checking for validity.
            pub unsafe fn from_event<E: AsRef<input_event>>(event: &E) -> &Self {
                let raw = event.as_ref();
                transmute(raw)
            }

            /// Reinterpret a mutable generic event without checking for validity.
            pub unsafe fn from_event_mut(event: &mut InputEvent) -> &mut Self {
                transmute(event)
            }

            /// A generic input event.
            pub fn into_event(self) -> InputEvent {
                unsafe {
                    transmute(self)
                }
            }

            /// A generic input event reference.
            pub fn as_event(&self) -> &InputEvent {
                unsafe {
                    transmute(self)
                }
            }

            /// A mutable generic input event reference.
            pub unsafe fn as_event_mut(&mut self) -> &mut InputEvent {
                transmute(self)
            }
        }
    };
    (@impl $name:ident $code:ident $kind:path, $codekind:ident, $valuekind:ident) => {
        event_impl! {
            @impl_event $name $code $kind, $codekind, $valuekind
        }

        impl AsRef<input_event> for $name {
            fn as_ref(&self) -> &input_event {
                let raw = self as *const _ as *const _;
                unsafe { &*raw }
            }
        }
    };
    ($(struct $name:ident : $kind:path { $code:ident: $codekind:ident, value: $valuekind:ident })*) => {
        $(
            event_impl! {
                @impl $name $code $kind, $codekind, $valuekind
            }
        )*
    };
}

/// A generic linux input event.
pub trait GenericEvent: AsRef<InputEvent> + AsRef<input_event> {
    /// The event kind.
    fn event_kind(&self) -> EventKind;
    /// The timestamp associated with the event.
    fn time(&self) -> &EventTime;
    /// The type code value of the event.
    fn code(&self) -> u16;
    /// The value associated with the event.
    fn value(&self) -> i32;

    /// Interprets a generic event into a concrete event type.
    fn with_event(event: InputEvent) -> Result<Self, RangeError> where
        Self: Clone,
    {
        Self::from_ref(&event).cloned()
    }

    /// Interprets a generic event into a concrete event type.
    ///
    /// # Safety
    ///
    /// The event must match `Self`'s [event kind](Self::event_kind) and have a valid [code](Self::code).
    unsafe fn with_event_unchecked(event: InputEvent) -> Self where
        Self: Clone,
    {
        match Self::from_ref(&event) {
            Ok(event) => event.clone(),
            Err(..) => unreachable_unchecked(),
        }
    }

    /// Interprets a generic event reference into a concrete event type.
    fn from_ref(event: &InputEvent) -> Result<&Self, RangeError>;
    /// Interprets a mutable generic event reference into a concrete event type.
    fn from_mut(event: &mut InputEvent) -> Result<&mut Self, RangeError>;
}

event_impl! {
    struct SynchronizeEvent : EventKind::Synchronize { kind: SynchronizeKind, value: i32 }
    struct KeyEvent : EventKind::Key { key: Key, value: KeyState }
    struct RelativeEvent : EventKind::Relative { axis: RelativeAxis, value: i32 }
    struct AbsoluteEvent : EventKind::Absolute { axis: AbsoluteAxis, value: i32 }
    struct SwitchEvent : EventKind::Switch { switch: SwitchKind, value: i32 }
    struct MiscEvent : EventKind::Misc { kind: MiscKind, value: i32 }
    struct LedEvent : EventKind::Led { led: LedKind, value: i32 }
    struct AutorepeatEvent : EventKind::Autorepeat { kind: AutorepeatKind, value: i32 }
    struct SoundEvent : EventKind::Sound { sound: SoundKind, value: i32 }
    struct UInputEvent : EventKind::UInput { code: UInputKind, value: i32 }
    struct InputEvent : Unknown { code: u16, value: i32 }
}

impl GenericEvent for InputEvent {
    fn event_kind(&self) -> EventKind { self.kind }
    fn time(&self) -> &EventTime { &self.time }
    fn code(&self) -> u16 { self.code }
    fn value(&self) -> i32 { self.value }

    fn from_ref(event: &InputEvent) -> Result<&Self, RangeError> {
        Ok(event)
    }

    fn from_mut(event: &mut InputEvent) -> Result<&mut Self, RangeError> {
        Ok(event)
    }

    fn with_event(event: InputEvent) -> Result<Self, RangeError> {
        Ok(event)
    }

    unsafe fn with_event_unchecked(event: InputEvent) -> Self {
        event
    }
}

impl AsRef<InputEvent> for InputEvent {
    fn as_ref(&self) -> &InputEvent {
        self
    }
}

impl InputEvent {
    /// Reinterprets a raw [`input_event`] reference.
    pub fn from_raw(event: &input_event) -> Result<&Self, RangeError> {
        EventKind::from_type(event.type_).map(|_| unsafe {
            Self::from_raw_unchecked(event)
        })
    }

    /// Reinterprets a raw [`input_event`] reference.
    ///
    /// # Safety
    ///
    /// The input event must have a valid [event kind](EventKind).
    pub unsafe fn from_raw_unchecked(event: &input_event) -> &Self {
        transmute(event)
    }

    /// Reinterprets a raw [`input_event`] into a mutable reference.
    pub fn from_raw_mut(event: &mut input_event) -> Result<&mut Self, RangeError> {
        EventKind::from_type(event.type_).map(move |_| unsafe {
            Self::from_raw_mut_unchecked(event)
        })
    }

    /// Reinterprets a raw [`input_event`] into a mutable reference.
    ///
    /// # Safety
    ///
    /// The input event must have a valid [event kind](EventKind).
    pub unsafe fn from_raw_mut_unchecked(event: &mut input_event) -> &mut Self {
        transmute(event)
    }

    /// Reinterprets the event as a raw [`input_event`].
    pub fn into_raw(self) -> input_event {
        unsafe {
            transmute(self)
        }
    }

    /// Reinterprets the event as a raw [`input_event`] reference.
    pub fn as_raw(&self) -> &input_event {
        unsafe {
            transmute(self)
        }
    }

    /// Reinterprets the event as a mutable raw [`input_event`].
    pub unsafe fn as_raw_mut(&mut self) -> &mut input_event {
        transmute(self)
    }
}

impl From<InputEvent> for input_event {
    fn from(event: InputEvent) -> Self {
        event.into_raw()
    }
}

impl<'a> From<&'a InputEvent> for &'a input_event {
    fn from(event: &'a InputEvent) -> Self {
        event.as_raw()
    }
}

impl TryFrom<input_event> for InputEvent {
    type Error = RangeError;

    fn try_from(event: input_event) -> Result<Self, Self::Error> {
        InputEvent::with_raw(event)
    }
}

impl<'a> TryFrom<&'a input_event> for &'a InputEvent {
    type Error = RangeError;

    fn try_from(event: &'a input_event) -> Result<Self, Self::Error> {
        InputEvent::from_raw(event)
    }
}

impl<'a> TryFrom<&'a mut input_event> for &'a mut InputEvent {
    type Error = RangeError;

    fn try_from(event: &'a mut input_event) -> Result<Self, Self::Error> {
        InputEvent::from_raw_mut(event)
    }
}

macro_rules! input_event_enum {
    ($($variant:ident($ty:ident),)*) => {
        /// An owned and typed input event.
        #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
        pub enum Event {
        $(
            #[allow(missing_docs)]
            $variant($ty),
        )*
            /// Unknown event type.
            Unknown(InputEvent),
        }

        /// A reference to an input event.
        #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
        pub enum EventRef<'a> {
        $(
            #[allow(missing_docs)]
            $variant(&'a $ty),
        )*
            /// Unknown event type.
            Unknown(&'a InputEvent),
        }

        /// A mutable reference to an input event.
        #[derive(PartialEq, Eq, Hash, Debug)]
        pub enum EventMut<'a> {
        $(
            #[allow(missing_docs)]
            $variant(&'a mut $ty),
        )*
            /// Unknown event type.
            Unknown(&'a mut InputEvent),
        }

        impl Event {
            /// Converts a generic [`InputEvent`] to a typed event.
            pub fn new(event: InputEvent) -> Result<Self, RangeError> {
                match event.kind {
                $(
                    EventKind::$variant => $ty::with_event(event).map(Event::$variant),
                )*
                    _ => Ok(Event::Unknown(event)),
                }
            }
        }

        impl<'a> EventRef<'a> {
            /// Wraps the generic [`InputEvent`] into an event.
            pub fn new(event: &'a InputEvent) -> Result<Self, RangeError> {
                match event.kind {
                $(
                    EventKind::$variant => $ty::from_ref(event).map(EventRef::$variant),
                )*
                    _ => Ok(EventRef::Unknown(event)),
                }
            }
        }

        impl<'a> EventMut<'a> {
            /// Wraps the generic [`InputEvent`] into a mutable event.
            pub fn new(event: &'a mut InputEvent) -> Result<Self, RangeError> {
                match event.kind {
                $(
                    EventKind::$variant => $ty::from_mut(event).map(EventMut::$variant),
                )*
                    _ => Ok(EventMut::Unknown(event)),
                }
            }
        }

        $(
        impl From<$ty> for Event {
            fn from(event: $ty) -> Self {
                Event::$variant(event)
            }
        }
        )*

        $(
        impl<'a> From<&'a $ty> for EventRef<'a> {
            fn from(event: &'a $ty) -> Self {
                EventRef::$variant(event)
            }
        }
        )*

        $(
        impl<'a> From<&'a mut $ty> for EventMut<'a> {
            fn from(event: &'a mut $ty) -> Self {
                EventMut::$variant(event)
            }
        }
        )*

        impl From<Event> for InputEvent {
            fn from(event: Event) -> Self {
                match event {
                $(
                    Event::$variant(ref event) => <&InputEvent as From<_>>::from(event).clone(),
                )*
                    Event::Unknown(event) => event,
                }
            }
        }

        impl<'a> From<&'a Event> for EventRef<'a> {
            fn from(event: &'a Event) -> Self {
                match *event {
                $(
                    Event::$variant(ref event) => EventRef::$variant(event),
                )*
                    Event::Unknown(ref event) => EventRef::Unknown(event),
                }
            }
        }

        impl<'a> From<&'a mut Event> for EventMut<'a> {
            fn from(event: &'a mut Event) -> Self {
                match *event {
                $(
                    Event::$variant(ref mut event) => EventMut::$variant(event),
                )*
                    Event::Unknown(ref mut event) => EventMut::Unknown(event),
                }
            }
        }

        impl<'a> From<EventRef<'a>> for &'a InputEvent {
            fn from(event: EventRef<'a>) -> Self {
                match event {
                $(
                    EventRef::$variant(event) => event.as_ref(),
                )*
                    EventRef::Unknown(event) => event,
                }
            }
        }

        impl<'a> From<&'a EventMut<'a>> for &'a InputEvent {
            fn from(event: &'a EventMut<'a>) -> Self {
                match *event {
                $(
                    EventMut::$variant(ref event) => event.as_ref(),
                )*
                    EventMut::Unknown(ref event) => event,
                }
            }
        }

        impl AsRef<InputEvent> for Event {
            fn as_ref(&self) -> &InputEvent {
                match *self {
                $(
                    Event::$variant(ref event) => event.as_ref(),
                )*
                    Event::Unknown(ref event) => event.as_ref(),
                }
            }
        }

        impl<'a> AsRef<InputEvent> for EventRef<'a> {
            fn as_ref(&self) -> &InputEvent {
                From::from(*self)
            }
        }

        impl<'a> AsRef<InputEvent> for EventMut<'a> {
            fn as_ref(&self) -> &InputEvent {
                From::from(self)
            }
        }
    };
}

input_event_enum! {
    Synchronize(SynchronizeEvent),
    Key(KeyEvent),
    Relative(RelativeEvent),
    Absolute(AbsoluteEvent),
    Switch(SwitchEvent),
    Misc(MiscEvent),
    Led(LedEvent),
    Autorepeat(AutorepeatEvent),
    Sound(SoundEvent),
    UInput(UInputEvent),
}
