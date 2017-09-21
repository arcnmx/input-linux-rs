//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/input-linux-rs/")]

extern crate input_linux_sys as sys;

use sys::input_event;

mod event_time;
pub use event_time::EventTime;

mod events;
pub use events::*;

mod keys;
pub use keys::Key;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum InputEvent {
    Synchronize {
        time: EventTime,
        kind: SynchronizeKind,
        value: i32,
    },
    Key {
        time: EventTime,
        key: Key,
        value: KeyState,
    },
    Relative {
        time: EventTime,
        axis: RelativeAxis,
        value: i32,
    },
    Absolute {
        time: EventTime,
        axis: AbsoluteAxis,
        value: i32,
    },
    Switch {
        time: EventTime,
        switch: SwitchKind,
        value: i32,
    },
    Misc {
        time: EventTime,
        kind: MiscKind,
        value: i32,
    },
    Led {
        time: EventTime,
        led: LedKind,
        value: i32,
    },
    Autorepeat {
        time: EventTime,
        kind: AutorepeatKind,
        value: i32,
    },
    Sound {
        time: EventTime,
        sound: SoundKind,
        value: i32,
    },
    Unknown {
        time: EventTime,
        kind: EventKind,
        code: u16,
        value: i32,
    },
}

impl InputEvent {
    pub fn new(event: &input_event) -> Result<Self, RangeError> {
        Ok(match EventKind::from_type(event.type_ as _)? {
            EventKind::Synchronize => InputEvent::Synchronize {
                time: event.time.into(),
                kind: SynchronizeKind::from_code(event.code)?,
                value: event.value,
            },
            EventKind::Key => InputEvent::Key {
                time: event.time.into(),
                key: Key::from_code(event.code)?,
                value: event.value.into(),
            },
            EventKind::Relative => InputEvent::Relative {
                time: event.time.into(),
                axis: RelativeAxis::from_code(event.code)?,
                value: event.value,
            },
            EventKind::Absolute => InputEvent::Absolute {
                time: event.time.into(),
                axis: AbsoluteAxis::from_code(event.code)?,
                value: event.value,
            },
            EventKind::Switch => InputEvent::Switch {
                time: event.time.into(),
                switch: SwitchKind::from_code(event.code)?,
                value: event.value,
            },
            EventKind::Misc => InputEvent::Misc {
                time: event.time.into(),
                kind: MiscKind::from_code(event.code)?,
                value: event.value,
            },
            EventKind::Led => InputEvent::Led {
                time: event.time.into(),
                led: LedKind::from_code(event.code)?,
                value: event.value,
            },
            EventKind::Autorepeat => InputEvent::Autorepeat {
                time: event.time.into(),
                kind: AutorepeatKind::from_code(event.code)?,
                value: event.value,
            },
            EventKind::Sound => InputEvent::Sound {
                time: event.time.into(),
                sound: SoundKind::from_code(event.code)?,
                value: event.value,
            },
            kind => InputEvent::Unknown {
                time: event.time.into(),
                kind: kind,
                code: event.code,
                value: event.value,
            },
        })
    }
}

impl<'a> From<&'a InputEvent> for input_event {
    fn from(event: &'a InputEvent) -> Self {
        match *event {
            InputEvent::Synchronize { time, kind, value } => input_event {
                time: time.into(),
                type_: EventKind::Synchronize as _,
                code: kind as _,
                value: value,
            },
            InputEvent::Key { time, key, value } => input_event {
                time: time.into(),
                type_: EventKind::Key as _,
                code: key as _,
                value: value.into(),
            },
            InputEvent::Relative { time, axis, value } => input_event {
                time: time.into(),
                type_: EventKind::Relative as _,
                code: axis as _,
                value: value,
            },
            InputEvent::Absolute { time, axis, value } => input_event {
                time: time.into(),
                type_: EventKind::Absolute as _,
                code: axis as _,
                value: value,
            },
            InputEvent::Switch { time, switch, value } => input_event {
                time: time.into(),
                type_: EventKind::Switch as _,
                code: switch as _,
                value: value,
            },
            InputEvent::Misc { time, kind, value } => input_event {
                time: time.into(),
                type_: EventKind::Misc as _,
                code: kind as _,
                value: value,
            },
            InputEvent::Led { time, led, value } => input_event {
                time: time.into(),
                type_: EventKind::Led as _,
                code: led as _,
                value: value,
            },
            InputEvent::Autorepeat { time, kind, value } => input_event {
                time: time.into(),
                type_: EventKind::Led as _,
                code: kind as _,
                value: value,
            },
            InputEvent::Sound { time, sound, value } => input_event {
                time: time.into(),
                type_: EventKind::Led as _,
                code: sound as _,
                value: value,
            },
            InputEvent::Unknown { time, kind, code, value } => input_event {
                time: time.into(),
                type_: kind as _,
                code: code,
                value: value,
            },
        }
    }
}
