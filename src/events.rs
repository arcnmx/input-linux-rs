use sys::input_event;
use ::{
    EventTime, RangeError, KeyState,
    EventKind, SynchronizeKind, Key, RelativeAxis, AbsoluteAxis,
    SwitchKind, MiscKind, LedKind, AutorepeatKind, SoundKind,
};

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct SynchronizeEvent {
    pub time: EventTime,
    event: EventKind,
    pub kind: SynchronizeKind,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct KeyEvent {
    pub time: EventTime,
    event: EventKind,
    pub key: Key,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct RelativeEvent {
    pub time: EventTime,
    event: EventKind,
    pub axis: RelativeAxis,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct AbsoluteEvent {
    pub time: EventTime,
    event: EventKind,
    pub axis: AbsoluteAxis,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct SwitchEvent {
    pub time: EventTime,
    event: EventKind,
    pub switch: SwitchKind,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct MiscEvent {
    pub time: EventTime,
    event: EventKind,
    pub kind: MiscKind,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct LedEvent {
    pub time: EventTime,
    event: EventKind,
    pub led: LedKind,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct AutorepeatEvent {
    pub time: EventTime,
    event: EventKind,
    pub kind: AutorepeatKind,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct SoundEvent {
    pub time: EventTime,
    event: EventKind,
    pub sound: SoundKind,
    pub value: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct InputEvent {
    pub time: EventTime,
    pub kind: EventKind,
    pub code: u16,
    pub value: i32,
}

impl KeyEvent {
    pub fn key_state(&self) -> KeyState {
        KeyState::from(self.value)
    }
}

macro_rules! event_impl {
    (@impl_event InputEvent $($tt:tt)*) => { };
    (@impl_event $name:ident $code:ident $kind:path, $codekind:ident) => {
        impl GenericEvent for $name {
            fn event_kind(&self) -> EventKind { $kind }
            fn time(&self) -> &EventTime { &self.time }
            fn code(&self) -> u16 { self.$code as _ }
            fn value(&self) -> i32 { self.value }

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
            pub unsafe fn from_event<E: AsRef<input_event>>(event: &E) -> &Self {
                let raw = event.as_ref() as *const _ as *const _;
                &*raw
            }

            pub unsafe fn from_event_mut(event: &mut InputEvent) -> &mut Self {
                let raw = event as *mut _ as *mut _;
                &mut *raw
            }

            pub fn as_event(&self) -> &InputEvent {
                let raw = self as *const _ as *const _;
                unsafe { &*raw }
            }

            pub unsafe fn as_event_mut(&mut self) -> &mut InputEvent {
                let raw = self as *mut _ as *mut _;
                &mut *raw
            }
        }
    };
    (@impl $name:ident $code:ident $kind:path, $codekind:ident) => {
        event_impl! {
            @impl_event $name $code $kind, $codekind
        }

        impl AsRef<input_event> for $name {
            fn as_ref(&self) -> &input_event {
                let raw = self as *const _ as *const _;
                unsafe { &*raw }
            }
        }
    };
    ($(struct $name:ident : $kind:path { $code:ident: $codekind:ident })*) => {
        $(
            event_impl! {
                @impl $name $code $kind, $codekind
            }
        )*
    };
}

pub trait GenericEvent: AsRef<InputEvent> + AsRef<input_event> {
    fn event_kind(&self) -> EventKind;
    fn time(&self) -> &EventTime;
    fn code(&self) -> u16;
    fn value(&self) -> i32;

    fn from_ref(event: &InputEvent) -> Result<&Self, RangeError>;
    fn from_mut(event: &mut InputEvent) -> Result<&mut Self, RangeError>;
}

event_impl! {
    struct SynchronizeEvent : EventKind::Synchronize { kind: SynchronizeKind }
    struct KeyEvent : EventKind::Key { key: Key }
    struct RelativeEvent : EventKind::Relative { axis: RelativeAxis }
    struct AbsoluteEvent : EventKind::Absolute { axis: AbsoluteAxis }
    struct SwitchEvent : EventKind::Switch { switch: SwitchKind }
    struct MiscEvent : EventKind::Misc { kind: MiscKind }
    struct LedEvent : EventKind::Led { led: LedKind }
    struct AutorepeatEvent : EventKind::Autorepeat { kind: AutorepeatKind }
    struct SoundEvent : EventKind::Sound { sound: SoundKind }
    struct InputEvent : Unknown { code: u16 }
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
}

impl AsRef<InputEvent> for InputEvent {
    fn as_ref(&self) -> &InputEvent {
        self
    }
}

impl InputEvent {
    pub fn from_raw(event: &input_event) -> Result<&Self, RangeError> {
        EventKind::from_type(event.type_).map(|_| {
            let raw = event as *const _ as *const _;
            unsafe { &*raw }
        })
    }

    pub fn from_raw_mut(event: &mut input_event) -> Result<&mut Self, RangeError> {
        EventKind::from_type(event.type_).map(|_| {
            let raw = event as *mut _ as *mut _;
            unsafe { &mut *raw }
        })
    }

    pub fn as_raw(&self) -> &input_event {
        let raw = self as *const _ as *const _;
        unsafe { &*raw }
    }

    pub unsafe fn as_raw_mut(&mut self) -> &mut input_event {
        let raw = self as *mut _ as *mut _;
        &mut *raw
    }
}

macro_rules! input_event_enum {
    ($($variant:ident($ty:ident),)*) => {
        #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
        pub enum Event {
        $(
            $variant($ty),
        )*
            Unknown(InputEvent),
        }

        #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
        pub enum EventRef<'a> {
        $(
            $variant(&'a $ty),
        )*
            Unknown(&'a InputEvent),
        }

        #[derive(PartialEq, Eq, Hash, Debug)]
        pub enum EventMut<'a> {
        $(
            $variant(&'a mut $ty),
        )*
            Unknown(&'a mut InputEvent),
        }

        impl Event {
            pub fn new(event: InputEvent) -> Result<Self, RangeError> {
                match event.kind {
                $(
                    EventKind::$variant => $ty::from_ref(&event).map(Clone::clone).map(Event::$variant),
                )*
                    _ => Ok(Event::Unknown(event)),
                }
            }
        }

        impl<'a> EventRef<'a> {
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
}
