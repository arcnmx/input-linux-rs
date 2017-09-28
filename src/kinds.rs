use std::mem::transmute;
use sys;

pub type RangeError = ();

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum InputProperty {
    Pointer = sys::INPUT_PROP_POINTER as _,
    Direct = sys::INPUT_PROP_DIRECT as _,
    ButtonPad = sys::INPUT_PROP_BUTTONPAD as _,
    SemiMultiTouch = sys::INPUT_PROP_SEMI_MT as _,
    TopButtonPad = sys::INPUT_PROP_TOPBUTTONPAD as _,
    PointingStick = sys::INPUT_PROP_POINTING_STICK as _,
    Accelerometer = sys::INPUT_PROP_ACCELEROMETER as _,
    Unknown07,
    Unknown08,
    Unknown09,
    Unknown0A,
    Unknown0B,
    Unknown0C,
    Unknown0D,
    Unknown0E,
    Unknown0F,
    Unknown10,
    Unknown11,
    Unknown12,
    Unknown13,
    Unknown14,
    Unknown15,
    Unknown16,
    Unknown17,
    Unknown18,
    Unknown19,
    Unknown1A,
    Unknown1B,
    Unknown1C,
    Unknown1D,
    Unknown1E,
    Unknown1F,
    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum EventKind {
    Synchronize = sys::EV_SYN as _,
    Key = sys::EV_KEY as _,
    Relative = sys::EV_REL as _,
    Absolute = sys::EV_ABS as _,
    Misc = sys::EV_MSC as _,
    Switch = sys::EV_SW as _,
    Unknown6,
    Unknown7,
    Unknown8,
    Unknown9,
    UnknownA,
    UnknownB,
    UnknownC,
    UnknownD,
    UnknownE,
    UnknownF,
    Unknown10,

    Led = sys::EV_LED as _,
    Sound = sys::EV_SND as _,
    Unknown13,

    Autorepeat = sys::EV_REP as _,
    ForceFeedback = sys::EV_FF as _,
    Power = sys::EV_PWR as _,
    ForceFeedbackStatus = sys::EV_FF_STATUS as _,
    Unknown18,
    Unknown19,
    Unknown1A,
    Unknown1B,
    Unknown1C,
    Unknown1D,
    Unknown1E,
    Unknown1F,

    Count,

    UInput = sys::EV_UINPUT as _,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum UInputKind {
    Unknown0 = 0,
    ForceFeedbackUpload = sys::UI_FF_UPLOAD as _,
    ForceFeedbackErase = sys::UI_FF_ERASE as _,
    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum SynchronizeKind {
    Report = sys::SYN_REPORT as _,
    Config = sys::SYN_CONFIG as _,
    MultitouchReport = sys::SYN_MT_REPORT as _,
    Dropped = sys::SYN_DROPPED as _,
    Unknown4,
    Unknown5,
    Unknown6,
    Unknown7,
    Unknown8,
    Unknown9,
    UnknownA,
    UnknownB,
    UnknownC,
    UnknownD,
    UnknownE,
    UnknownF,

    Count,
}

// XXX: sure would be nice if Rust knew these weren't overlapping and allowed #[repr(i32)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum KeyState {
    Released, // 0
    Pressed, // 1
    Autorepeat, // 2
    Unknown(i32),
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum RelativeAxis {
    X = sys::REL_X as _,
    Y = sys::REL_Y as _,
    Z = sys::REL_Z as _,
    RX = sys::REL_RX as _,
    RY = sys::REL_RY as _,
    RZ = sys::REL_RZ as _,
    HorizontalWheel = sys::REL_HWHEEL as _,
    Dial = sys::REL_DIAL as _,
    Wheel = sys::REL_WHEEL as _,
    Misc = sys::REL_MISC as _,
    UnknownA,
    UnknownB,
    UnknownC,
    UnknownD,
    UnknownE,
    UnknownF,

    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum AbsoluteAxis {
    X = sys::ABS_X as _,
    Y = sys::ABS_Y as _,
    Z = sys::ABS_Z as _,
    RX = sys::ABS_RX as _,
    RY = sys::ABS_RY as _,
    RZ = sys::ABS_RZ as _,
    Throttle = sys::ABS_THROTTLE as _,
    Rudder = sys::ABS_RUDDER as _,
    Wheel = sys::ABS_WHEEL as _,
    Gas = sys::ABS_GAS as _,
    Brake = sys::ABS_BRAKE as _,
    UnknownB,
    UnknownC,
    UnknownD,
    UnknownE,
    UnknownF,

    Hat0X = sys::ABS_HAT0X as _,
    Hat0Y = sys::ABS_HAT0Y as _,
    Hat1X = sys::ABS_HAT1X as _,
    Hat1Y = sys::ABS_HAT1Y as _,
    Hat2X = sys::ABS_HAT2X as _,
    Hat2Y = sys::ABS_HAT2Y as _,
    Hat3X = sys::ABS_HAT3X as _,
    Hat3Y = sys::ABS_HAT3Y as _,
    Pressure = sys::ABS_PRESSURE as _,
    Distance = sys::ABS_DISTANCE as _,
    TiltX = sys::ABS_TILT_X as _,
    TiltY = sys::ABS_TILT_Y as _,
    ToolWidth = sys::ABS_TOOL_WIDTH as _,
    Unknown1D,
    Unknown1E,
    Unknown1F,

    Volume = sys::ABS_VOLUME as _,
    Unknown21,
    Unknown22,
    Unknown23,
    Unknown24,
    Unknown25,
    Unknown26,
    Unknown27,

    Misc = sys::ABS_MISC as _,
    Unknown29,
    Unknown2A,
    Unknown2B,
    Unknown2C,
    Unknown2D,
    Unknown2E,

    /// MT slot being modified
    MultitouchSlot = sys::ABS_MT_SLOT as _,
    /// Major axis of touching ellipse
    MultitouchTouchMajor = sys::ABS_MT_TOUCH_MAJOR as _,
    /// Minor axis (omit if circular)
    MultitouchTouchMinor = sys::ABS_MT_TOUCH_MINOR as _,
    /// Major axis of approaching ellipse
    MultitouchWidthMajor = sys::ABS_MT_WIDTH_MAJOR as _,
    /// Minor axis (omit if circular)
    MultitouchWidthMinor = sys::ABS_MT_WIDTH_MINOR as _,
    /// Ellipse orientation
    MultitouchOrientation = sys::ABS_MT_ORIENTATION as _,
    /// Center X touch position
    MultitouchPositionX = sys::ABS_MT_POSITION_X as _,
    /// Center Y touch position
    MultitouchPositionY = sys::ABS_MT_POSITION_Y as _,
    /// Type of touching device
    MultitouchToolType = sys::ABS_MT_TOOL_TYPE as _,
    /// Group a set of packets as a blob
    MultitouchBlobId = sys::ABS_MT_BLOB_ID as _,
    /// Unique ID of initiated contact
    MultitouchTrackingId = sys::ABS_MT_TRACKING_ID as _,
    /// Pressure on contact area
    MultitouchPressure = sys::ABS_MT_PRESSURE as _,
    /// Contact hover distance
    MultitouchDistance = sys::ABS_MT_DISTANCE as _,
    /// Center X tool position
    MultitouchToolX = sys::ABS_MT_TOOL_X as _,
    /// Center Y tool position
    MultitouchToolY = sys::ABS_MT_TOOL_Y as _,
    Unknown3E,
    Unknown3F,

    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum SwitchKind {
    /// set = lid shut
    Lid = sys::SW_LID as _,
    /// set = tablet mode
    TabletMode = sys::SW_TABLET_MODE as _,
    /// set = inserted
    HeadphoneInsert = sys::SW_HEADPHONE_INSERT as _,
    /// set = radio enabled
    RfKillAll = sys::SW_RFKILL_ALL as _,
    // Radio = sys::SW_RADIO as _ = RfKillAll,
    /// set = inserted
    MicrophoneInsert = sys::SW_MICROPHONE_INSERT as _,
    /// set = plugged into dock
    Dock = sys::SW_DOCK as _,
    /// set = inserted
    LineoutInsert = sys::SW_LINEOUT_INSERT as _,
    /// set = mechanical switch set
    JackPhysicalInsert = sys::SW_JACK_PHYSICAL_INSERT as _,
    /// set = inserted
    VideoOutInsert = sys::SW_VIDEOOUT_INSERT as _,
    /// set = lens covered
    CameraLensCover = sys::SW_CAMERA_LENS_COVER as _,
    /// set = keypad slide out
    KeypadSlide = sys::SW_KEYPAD_SLIDE as _,
    /// set = front proximity sensor active
    FrontProximity = sys::SW_FRONT_PROXIMITY as _,
    /// set = rotate locked/disabled
    RotateLock = sys::SW_ROTATE_LOCK as _,
    /// set = inserted
    LineInInsert = sys::SW_LINEIN_INSERT as _,
    /// set = device disabled
    MuteDevice = sys::SW_MUTE_DEVICE as _,
    /// set = pen inserted
    PenInserted = sys::SW_PEN_INSERTED as _,

    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum MiscKind {
    /// Serial number, only exported for tablets ("Transducer Serial Number")
    Serial = sys::MSC_SERIAL as _,
    /// Only used by the PowerMate driver, right now.
    PulseLed = sys::MSC_PULSELED as _,
    /// Completely unused
    Gesture = sys::MSC_GESTURE as _,
    /// "Raw" event, rarely used.
    Raw = sys::MSC_RAW as _,
    /// Key scancode
    Scancode = sys::MSC_SCAN as _,
    /// Completely unused
    Timestamp = sys::MSC_TIMESTAMP as _,
    Unknown6,
    Unknown7,

    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum LedKind {
    NumLock = sys::LED_NUML as _,
    CapsLock = sys::LED_CAPSL as _,
    ScrollLock = sys::LED_SCROLLL as _,
    Compose = sys::LED_COMPOSE as _,
    Kana = sys::LED_KANA as _,
    Sleep = sys::LED_SLEEP as _,
    Suspend = sys::LED_SUSPEND as _,
    Mute = sys::LED_MUTE as _,
    Misc = sys::LED_MISC as _,
    Mail = sys::LED_MAIL as _,
    Charging = sys::LED_CHARGING as _,
    UnknownB,
    UnknownC,
    UnknownD,
    UnknownE,
    UnknownF,

    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum AutorepeatKind {
    Delay = sys::REP_DELAY as _,
    Period = sys::REP_PERIOD as _,

    Count,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum SoundKind {
    Click = sys::SND_CLICK as _,
    Bell = sys::SND_BELL as _,
    Tone = sys::SND_TONE as _,
    Unknown3,
    Unknown4,
    Unknown5,
    Unknown6,
    Unknown7,

    Count,
}

impl InputProperty {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x1f => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for InputProperty {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl EventKind {
    pub fn from_type(code: u16) -> Result<Self, RangeError> {
        const EV_UINPUT: u16 = sys::EV_UINPUT as _;

        match code {
            0...0x1f | EV_UINPUT => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for EventKind {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_type(code)
    }
}

impl UInputKind {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x02 => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for UInputKind {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl SynchronizeKind {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x0f => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for SynchronizeKind {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl From<i32> for KeyState {
    fn from(key: i32) -> Self {
        match key {
            0 => KeyState::Released,
            1 => KeyState::Pressed,
            2 => KeyState::Autorepeat,
            key => KeyState::Unknown(key),
        }
    }
}

impl From<KeyState> for i32 {
    fn from(k: KeyState) -> Self {
        match k {
            KeyState::Released => 0,
            KeyState::Pressed => 1,
            KeyState::Autorepeat => 2,
            KeyState::Unknown(key) => key,
        }
    }
}

impl RelativeAxis {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x0f => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for RelativeAxis {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl AbsoluteAxis {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x3f => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for AbsoluteAxis {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl SwitchKind {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x0f => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for SwitchKind {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl MiscKind {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x07 => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for Misc {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl LedKind {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x0f => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for LedKind {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl AutorepeatKind {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x01 => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for AutorepeatKind {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl SoundKind {
    pub fn from_code(code: u16) -> Result<Self, RangeError> {
        match code {
            0...0x07 => Ok(unsafe { transmute(code) }),
            _ => return Err(Default::default()),
        }
    }
}

#[cfg(feature = "unstable")]
impl TryFrom<u16> for SoundKind {
    type Error = RangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}
