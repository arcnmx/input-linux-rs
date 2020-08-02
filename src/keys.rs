use sys;

/// Keys and Buttons
///
/// Most of the keys/buttons are modeled after USB HUT 1.12 (see http://www.usb.org/developers/hidpage).
///
/// ## Comment Abbreviations
/// AC - Application Control
/// AL - Application Launch Button
/// SC - System Control
#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "with-serde", derive(Deserialize, Serialize))]
#[allow(missing_docs)]
pub enum Key {
    Reserved = sys::KEY_RESERVED as _,
    Esc = sys::KEY_ESC as _,
    Num1 = sys::KEY_1 as _,
    Num2 = sys::KEY_2 as _,
    Num3 = sys::KEY_3 as _,
    Num4 = sys::KEY_4 as _,
    Num5 = sys::KEY_5 as _,
    Num6 = sys::KEY_6 as _,
    Num7 = sys::KEY_7 as _,
    Num8 = sys::KEY_8 as _,
    Num9 = sys::KEY_9 as _,
    Num0 = sys::KEY_0 as _,
    Minus = sys::KEY_MINUS as _,
    Equal = sys::KEY_EQUAL as _,
    Backspace = sys::KEY_BACKSPACE as _,
    Tab = sys::KEY_TAB as _,
    Q = sys::KEY_Q as _,
    W = sys::KEY_W as _,
    E = sys::KEY_E as _,
    R = sys::KEY_R as _,
    T = sys::KEY_T as _,
    Y = sys::KEY_Y as _,
    U = sys::KEY_U as _,
    I = sys::KEY_I as _,
    O = sys::KEY_O as _,
    P = sys::KEY_P as _,
    LeftBrace = sys::KEY_LEFTBRACE as _,
    RightBrace = sys::KEY_RIGHTBRACE as _,
    Enter = sys::KEY_ENTER as _,
    LeftCtrl = sys::KEY_LEFTCTRL as _,
    A = sys::KEY_A as _,
    S = sys::KEY_S as _,
    D = sys::KEY_D as _,
    F = sys::KEY_F as _,
    G = sys::KEY_G as _,
    H = sys::KEY_H as _,
    J = sys::KEY_J as _,
    K = sys::KEY_K as _,
    L = sys::KEY_L as _,
    Semicolon = sys::KEY_SEMICOLON as _,
    Apostrophe = sys::KEY_APOSTROPHE as _,
    Grave = sys::KEY_GRAVE as _,
    LeftShift = sys::KEY_LEFTSHIFT as _,
    Backslash = sys::KEY_BACKSLASH as _,
    Z = sys::KEY_Z as _,
    X = sys::KEY_X as _,
    C = sys::KEY_C as _,
    V = sys::KEY_V as _,
    B = sys::KEY_B as _,
    N = sys::KEY_N as _,
    M = sys::KEY_M as _,
    Comma = sys::KEY_COMMA as _,
    Dot = sys::KEY_DOT as _,
    Slash = sys::KEY_SLASH as _,
    RightShift = sys::KEY_RIGHTSHIFT as _,
    KpAsterisk = sys::KEY_KPASTERISK as _,
    LeftAlt = sys::KEY_LEFTALT as _,
    Space = sys::KEY_SPACE as _,
    CapsLock = sys::KEY_CAPSLOCK as _,
    F1 = sys::KEY_F1 as _,
    F2 = sys::KEY_F2 as _,
    F3 = sys::KEY_F3 as _,
    F4 = sys::KEY_F4 as _,
    F5 = sys::KEY_F5 as _,
    F6 = sys::KEY_F6 as _,
    F7 = sys::KEY_F7 as _,
    F8 = sys::KEY_F8 as _,
    F9 = sys::KEY_F9 as _,
    F10 = sys::KEY_F10 as _,
    NumLock = sys::KEY_NUMLOCK as _,
    ScrollLock = sys::KEY_SCROLLLOCK as _,
    Kp7 = sys::KEY_KP7 as _,
    Kp8 = sys::KEY_KP8 as _,
    Kp9 = sys::KEY_KP9 as _,
    KpMinus = sys::KEY_KPMINUS as _,
    Kp4 = sys::KEY_KP4 as _,
    Kp5 = sys::KEY_KP5 as _,
    Kp6 = sys::KEY_KP6 as _,
    KpPlus = sys::KEY_KPPLUS as _,
    Kp1 = sys::KEY_KP1 as _,
    Kp2 = sys::KEY_KP2 as _,
    Kp3 = sys::KEY_KP3 as _,
    Kp0 = sys::KEY_KP0 as _,
    KpDot = sys::KEY_KPDOT as _,

    Unknown54,

    ZenkakuHankaku = sys::KEY_ZENKAKUHANKAKU as _,
    NonUsBackslashAndPipe = sys::KEY_102ND as _,
    F11 = sys::KEY_F11 as _,
    F12 = sys::KEY_F12 as _,
    Ro = sys::KEY_RO as _,
    Katakana = sys::KEY_KATAKANA as _,
    Hiragana = sys::KEY_HIRAGANA as _,
    Henkan = sys::KEY_HENKAN as _,
    KatakanaHiragana = sys::KEY_KATAKANAHIRAGANA as _,
    Muhenkan = sys::KEY_MUHENKAN as _,
    KpJpComma = sys::KEY_KPJPCOMMA as _,
    KpEnter = sys::KEY_KPENTER as _,
    RightCtrl = sys::KEY_RIGHTCTRL as _,
    KpSlash = sys::KEY_KPSLASH as _,
    Sysrq = sys::KEY_SYSRQ as _,
    RightAlt = sys::KEY_RIGHTALT as _,
    LineFeed = sys::KEY_LINEFEED as _,
    Home = sys::KEY_HOME as _,
    Up = sys::KEY_UP as _,
    PageUp = sys::KEY_PAGEUP as _,
    Left = sys::KEY_LEFT as _,
    Right = sys::KEY_RIGHT as _,
    End = sys::KEY_END as _,
    Down = sys::KEY_DOWN as _,
    PageDown = sys::KEY_PAGEDOWN as _,
    Insert = sys::KEY_INSERT as _,
    Delete = sys::KEY_DELETE as _,
    Macro = sys::KEY_MACRO as _,

    Mute = sys::KEY_MUTE as _,
    VolumeDown = sys::KEY_VOLUMEDOWN as _,
    VolumeUp = sys::KEY_VOLUMEUP as _,
    /// SC System Power Down
    Power = sys::KEY_POWER as _,
    KpEqual = sys::KEY_KPEQUAL as _,
    KpPlusMinus = sys::KEY_KPPLUSMINUS as _,
    Pause = sys::KEY_PAUSE as _,
    /// AL Compiz Scale (Expose)
    Scale = sys::KEY_SCALE as _,

    KpComma = sys::KEY_KPCOMMA as _,
    /// KeyHangeul / KeyHanguel
    Hangul = sys::KEY_HANGEUL as _,
    // KeyHangeul = KeyHangul
    // KeyHanguel = KeyHangul
    Hanja = sys::KEY_HANJA as _,
    Yen = sys::KEY_YEN as _,
    LeftMeta = sys::KEY_LEFTMETA as _,
    RightMeta = sys::KEY_RIGHTMETA as _,
    Compose = sys::KEY_COMPOSE as _,

    /// AC Stop
    Stop = sys::KEY_STOP as _,
    Again = sys::KEY_AGAIN as _,
    /// AC Properties
    Props = sys::KEY_PROPS as _,
    /// AC Undo
    Undo = sys::KEY_UNDO as _,
    Front = sys::KEY_FRONT as _,
    /// AC Copy
    Copy = sys::KEY_COPY as _,
    /// AC Open
    Open = sys::KEY_OPEN as _,
    /// AC Paste
    Paste = sys::KEY_PASTE as _,
    /// AC Search
    Find = sys::KEY_FIND as _,
    /// AC Cut
    Cut = sys::KEY_CUT as _,
    /// AL Integrated Help Center
    Help = sys::KEY_HELP as _,
    /// Menu (show menu)
    Menu = sys::KEY_MENU as _,
    /// AL Calculator
    Calc = sys::KEY_CALC as _,
    Setup = sys::KEY_SETUP as _,
    /// SC System Sleep
    Sleep = sys::KEY_SLEEP as _,
    /// System Wake Up
    Wakeup = sys::KEY_WAKEUP as _,
    /// AL Local Machine Browser
    File = sys::KEY_FILE as _,
    SendFile = sys::KEY_SENDFILE as _,
    DeleteFile = sys::KEY_DELETEFILE as _,
    Xfer = sys::KEY_XFER as _,
    Prog1 = sys::KEY_PROG1 as _,
    Prog2 = sys::KEY_PROG2 as _,
    /// AL Internet Browser
    WWW = sys::KEY_WWW as _,
    MSDOS = sys::KEY_MSDOS as _,
    /// AL Terminal Lock/Screensaver
    /// KeyScreenLock
    Coffee = sys::KEY_COFFEE as _,
    // KeyScreenLock = KeyCoffee,
    /// Display orientation for e.g. tablets (aka KeyDirectionKey)
    RotateDisplay = sys::KEY_ROTATE_DISPLAY as _,
    // KeyDirectionKey = KeyRotateDisplay
    CycleWindows = sys::KEY_CYCLEWINDOWS as _,
    Mail = sys::KEY_MAIL as _,
    /// AC Bookmarks
    Bookmarks = sys::KEY_BOOKMARKS as _,
    Computer = sys::KEY_COMPUTER as _,
    /// AC Back
    Back = sys::KEY_BACK as _,
    /// AC Forward
    Forward = sys::KEY_FORWARD as _,
    CloseCD = sys::KEY_CLOSECD as _,
    EjectCD = sys::KEY_EJECTCD as _,
    EjectCloseCD = sys::KEY_EJECTCLOSECD as _,
    NextSong = sys::KEY_NEXTSONG as _,
    PlayPause = sys::KEY_PLAYPAUSE as _,
    PreviousSong = sys::KEY_PREVIOUSSONG as _,
    StopCD = sys::KEY_STOPCD as _,
    Record = sys::KEY_RECORD as _,
    Rewind = sys::KEY_REWIND as _,
    /// Media Select Telephone
    Phone = sys::KEY_PHONE as _,
    Iso = sys::KEY_ISO as _,
    /// AL Consumer Control Configuration
    Config = sys::KEY_CONFIG as _,
    /// AC Home
    Homepage = sys::KEY_HOMEPAGE as _,
    /// AC Refresh
    Refresh = sys::KEY_REFRESH as _,
    /// AC Exit
    Exit = sys::KEY_EXIT as _,
    Move = sys::KEY_MOVE as _,
    Edit = sys::KEY_EDIT as _,
    ScrollUp = sys::KEY_SCROLLUP as _,
    ScrollDown = sys::KEY_SCROLLDOWN as _,
    KpLeftParen = sys::KEY_KPLEFTPAREN as _,
    KpRightParen = sys::KEY_KPRIGHTPAREN as _,
    /// AC New
    New = sys::KEY_NEW as _,
    /// AC Redo/Repeat
    Redo = sys::KEY_REDO as _,

    F13 = sys::KEY_F13 as _,
    F14 = sys::KEY_F14 as _,
    F15 = sys::KEY_F15 as _,
    F16 = sys::KEY_F16 as _,
    F17 = sys::KEY_F17 as _,
    F18 = sys::KEY_F18 as _,
    F19 = sys::KEY_F19 as _,
    F20 = sys::KEY_F20 as _,
    F21 = sys::KEY_F21 as _,
    F22 = sys::KEY_F22 as _,
    F23 = sys::KEY_F23 as _,
    F24 = sys::KEY_F24 as _,

    UnknownC3,
    UnknownC4,
    UnknownC5,
    UnknownC6,
    UnknownC7,

    PlayCD = sys::KEY_PLAYCD as _,
    PauseCD = sys::KEY_PAUSECD as _,
    Prog3 = sys::KEY_PROG3 as _,
    Prog4 = sys::KEY_PROG4 as _,
    /// AL Dashboard
    Dashboard = sys::KEY_DASHBOARD as _,
    Suspend = sys::KEY_SUSPEND as _,
    /// AC Close
    Close = sys::KEY_CLOSE as _,
    Play = sys::KEY_PLAY as _,
    FastForward = sys::KEY_FASTFORWARD as _,
    BassBoost = sys::KEY_BASSBOOST as _,
    /// AC Print
    Print = sys::KEY_PRINT as _,
    Hp = sys::KEY_HP as _,
    Camera = sys::KEY_CAMERA as _,
    Sound = sys::KEY_SOUND as _,
    Question = sys::KEY_QUESTION as _,
    Email = sys::KEY_EMAIL as _,
    Chat = sys::KEY_CHAT as _,
    Search = sys::KEY_SEARCH as _,
    Connect = sys::KEY_CONNECT as _,
    /// AL Checkbook/Finance
    Finance = sys::KEY_FINANCE as _,
    Sport = sys::KEY_SPORT as _,
    Shop = sys::KEY_SHOP as _,
    Alterase = sys::KEY_ALTERASE as _,
    /// AC Cancel
    Cancel = sys::KEY_CANCEL as _,
    BrightnessDown = sys::KEY_BRIGHTNESSDOWN as _,
    BrightnessUp = sys::KEY_BRIGHTNESSUP as _,
    Media = sys::KEY_MEDIA as _,

    /// Cycle between available video outputs (Monitor/LCD/TV-out/etc)
    SwitchVideoMode = sys::KEY_SWITCHVIDEOMODE as _,
    IllumToggle = sys::KEY_KBDILLUMTOGGLE as _,
    IllumDown = sys::KEY_KBDILLUMDOWN as _,
    IllumUp = sys::KEY_KBDILLUMUP as _,

    /// AC Send
    Send = sys::KEY_SEND as _,
    /// AC Reply
    Reply = sys::KEY_REPLY as _,
    /// AC Forward Msg
    ForwardMail = sys::KEY_FORWARDMAIL as _,
    /// AC Save
    Save = sys::KEY_SAVE as _,
    Documents = sys::KEY_DOCUMENTS as _,

    Battery = sys::KEY_BATTERY as _,

    Bluetooth = sys::KEY_BLUETOOTH as _,
    WLAN = sys::KEY_WLAN as _,
    UWB = sys::KEY_UWB as _,

    Unknown = sys::KEY_UNKNOWN as _,

    /// drive next video source
    VideoNext = sys::KEY_VIDEO_NEXT as _,
    /// drive previous video source
    VideoPrev = sys::KEY_VIDEO_PREV as _,
    /// brightness up, after max is min
    BrightnessCycle = sys::KEY_BRIGHTNESS_CYCLE as _,
    /// Set Auto Brightness: manual brightness control is off, rely on ambient
    /// (aka KeyBrightnessZero)
    BrightnessAuto = sys::KEY_BRIGHTNESS_AUTO as _,
    // KeyBrightnessZero = KeyBrightnessAuto
    /// display device to off state
    DisplayOff = sys::KEY_DISPLAY_OFF as _,

    /// Wireless WAN (LTE, UMTS, GSM, etc.)
    /// (aka KeyWiMAX)
    WWAN = sys::KEY_WWAN as _,
    // KeyWiMAX = KeyWWAN
    /// Key that controls all radios
    Rfkill = sys::KEY_RFKILL as _,

    /// Mute / unmute the microphone
    MicMute = sys::KEY_MICMUTE as _,

    UnknownF9,
    UnknownFA,
    UnknownFB,
    UnknownFC,
    UnknownFD,
    UnknownFE,

    /// Code 255 is reserved for special needs of AT keyboard driver
    ReservedFF = 0xff,

    //ButtonMisc = sys::BTN_MISC as _,
    Button0 = sys::BTN_0 as _,
    Button1 = sys::BTN_1 as _,
    Button2 = sys::BTN_2 as _,
    Button3 = sys::BTN_3 as _,
    Button4 = sys::BTN_4 as _,
    Button5 = sys::BTN_5 as _,
    Button6 = sys::BTN_6 as _,
    Button7 = sys::BTN_7 as _,
    Button8 = sys::BTN_8 as _,
    Button9 = sys::BTN_9 as _,

    Unknown10A,
    Unknown10B,
    Unknown10C,
    Unknown10D,
    Unknown10E,
    Unknown10F,

    //ButtonMouse = sys::BTN_MOUSE as _,
    ButtonLeft = sys::BTN_LEFT as _,
    ButtonRight = sys::BTN_RIGHT as _,
    ButtonMiddle = sys::BTN_MIDDLE as _,
    ButtonSide = sys::BTN_SIDE as _,
    ButtonExtra = sys::BTN_EXTRA as _,
    ButtonForward = sys::BTN_FORWARD as _,
    ButtonBack = sys::BTN_BACK as _,
    ButtonTask = sys::BTN_TASK as _,

    Unknown118,
    Unknown119,
    Unknown11A,
    Unknown11B,
    Unknown11C,
    Unknown11D,
    Unknown11E,
    Unknown11F,

    //ButtonJoystick = sys::BTN_JOYSTICK as _,
    ButtonTrigger = sys::BTN_TRIGGER as _,
    ButtonThumb = sys::BTN_THUMB as _,
    ButtonThumb2 = sys::BTN_THUMB2 as _,
    ButtonTop = sys::BTN_TOP as _,
    ButtonTop2 = sys::BTN_TOP2 as _,
    ButtonPinkie = sys::BTN_PINKIE as _,
    ButtonBase = sys::BTN_BASE as _,
    ButtonBase2 = sys::BTN_BASE2 as _,
    ButtonBase3 = sys::BTN_BASE3 as _,
    ButtonBase4 = sys::BTN_BASE4 as _,
    ButtonBase5 = sys::BTN_BASE5 as _,
    ButtonBase6 = sys::BTN_BASE6 as _,

    Unknown12C,
    Unknown12D,
    Unknown12E,

    ButtonDead = sys::BTN_DEAD as _,

    //ButtonGamepad = sys::BTN_GAMEPAD as _,
    /// aka ButtonA
    ButtonSouth = sys::BTN_SOUTH as _,
    // ButtonA = ButtonSouth
    /// aka ButtonB
    ButtonEast = sys::BTN_EAST as _,
    // ButtonB = ButtonEast
    ButtonC = sys::BTN_C as _,
    /// aka ButtonX
    ButtonNorth = sys::BTN_NORTH as _,
    // ButtonX = ButtonNorth
    /// aka ButtonY
    ButtonWest = sys::BTN_WEST as _,
    // ButtonY = ButtonWest
    ButtonZ = sys::BTN_Z as _,
    ButtonTL = sys::BTN_TL as _,
    ButtonTR = sys::BTN_TR as _,
    ButtonTL2 = sys::BTN_TL2 as _,
    ButtonTR2 = sys::BTN_TR2 as _,
    ButtonSelect = sys::BTN_SELECT as _,
    ButtonStart = sys::BTN_START as _,
    ButtonMode = sys::BTN_MODE as _,
    ButtonThumbl = sys::BTN_THUMBL as _,
    ButtonThumbr = sys::BTN_THUMBR as _,

    Unknown13F,

    //ButtonDigi = sys::BTN_DIGI as _,
    ButtonToolPen = sys::BTN_TOOL_PEN as _,
    ButtonToolRubber = sys::BTN_TOOL_RUBBER as _,
    ButtonToolBrush = sys::BTN_TOOL_BRUSH as _,
    ButtonToolPencil = sys::BTN_TOOL_PENCIL as _,
    ButtonToolAirbrush = sys::BTN_TOOL_AIRBRUSH as _,
    ButtonToolFinger = sys::BTN_TOOL_FINGER as _,
    ButtonToolMouse = sys::BTN_TOOL_MOUSE as _,
    ButtonToolLens = sys::BTN_TOOL_LENS as _,
    /// Five fingers on trackpad
    ButtonToolQuintTap = sys::BTN_TOOL_QUINTTAP as _,
    ButtonStylus3 = sys::BTN_STYLUS3 as _,
    ButtonTouch = sys::BTN_TOUCH as _,
    ButtonStylus = sys::BTN_STYLUS as _,
    ButtonStylus2 = sys::BTN_STYLUS2 as _,
    ButtonToolDoubleTap = sys::BTN_TOOL_DOUBLETAP as _,
    ButtonToolTripleTap = sys::BTN_TOOL_TRIPLETAP as _,
    /// Four fingers on trackpad
    ButtonToolQuadtap = sys::BTN_TOOL_QUADTAP as _,

    ButtonWheel = sys::BTN_WHEEL as _,
    //ButtonGearDown = sys::BTN_GEAR_DOWN as _,
    ButtonGearUp = sys::BTN_GEAR_UP as _,

    Unknown152,
    Unknown153,
    Unknown154,
    Unknown155,
    Unknown156,
    Unknown157,
    Unknown158,
    Unknown159,
    Unknown15A,
    Unknown15B,
    Unknown15C,
    Unknown15D,
    Unknown15E,
    Unknown15F,

    Ok = sys::KEY_OK as _,
    Select = sys::KEY_SELECT as _,
    Goto = sys::KEY_GOTO as _,
    Clear = sys::KEY_CLEAR as _,
    Power2 = sys::KEY_POWER2 as _,
    Option = sys::KEY_OPTION as _,
    /// AL OEM Features/Tips/Tutorial
    Info = sys::KEY_INFO as _,
    Time = sys::KEY_TIME as _,
    Vendor = sys::KEY_VENDOR as _,
    Archive = sys::KEY_ARCHIVE as _,
    /// Media Select Program Guide
    Program = sys::KEY_PROGRAM as _,
    Channel = sys::KEY_CHANNEL as _,
    Favorites = sys::KEY_FAVORITES as _,
    EPG = sys::KEY_EPG as _,
    /// Media Select Home
    PVR = sys::KEY_PVR as _,
    MHP = sys::KEY_MHP as _,
    Language = sys::KEY_LANGUAGE as _,
    Title = sys::KEY_TITLE as _,
    Subtitle = sys::KEY_SUBTITLE as _,
    Angle = sys::KEY_ANGLE as _,
    FullScreen = sys::KEY_FULL_SCREEN as _,
    Mode = sys::KEY_MODE as _,
    Keyboard = sys::KEY_KEYBOARD as _,
    AspectRatio = sys::KEY_ASPECT_RATIO as _,
    /// Media Select Computer
    PC = sys::KEY_PC as _,
    /// Media Select TV
    TV = sys::KEY_TV as _,
    /// Media Select Cable
    TV2 = sys::KEY_TV2 as _,
    /// Media Select VCR
    VCR = sys::KEY_VCR as _,
    /// VCR Plus
    VCR2 = sys::KEY_VCR2 as _,
    /// Media Select Satellite
    Sat = sys::KEY_SAT as _,
    Sat2 = sys::KEY_SAT2 as _,
    /// Media Select CD
    CD = sys::KEY_CD as _,
    /// Media Select Tape
    Tape = sys::KEY_TAPE as _,
    Radio = sys::KEY_RADIO as _,
    /// Media Select Tuner
    Tuner = sys::KEY_TUNER as _,
    Player = sys::KEY_PLAYER as _,
    Text = sys::KEY_TEXT as _,
    /// Media Select DVD
    Dvd = sys::KEY_DVD as _,
    Aux = sys::KEY_AUX as _,
    Mp3 = sys::KEY_MP3 as _,
    /// AL Audio Browser
    Audio = sys::KEY_AUDIO as _,
    /// AL Movie Browser
    Video = sys::KEY_VIDEO as _,
    Directory = sys::KEY_DIRECTORY as _,
    List = sys::KEY_LIST as _,
    /// Media Select Messages
    Memo = sys::KEY_MEMO as _,
    Calendar = sys::KEY_CALENDAR as _,
    Red = sys::KEY_RED as _,
    Green = sys::KEY_GREEN as _,
    Yellow = sys::KEY_YELLOW as _,
    Blue = sys::KEY_BLUE as _,
    /// Channel Increment
    ChannelUp = sys::KEY_CHANNELUP as _,
    /// Channel Decrement
    ChannelDown = sys::KEY_CHANNELDOWN as _,
    First = sys::KEY_FIRST as _,
    /// Recall Last
    Last = sys::KEY_LAST as _,
    Ab = sys::KEY_AB as _,
    Next = sys::KEY_NEXT as _,
    Restart = sys::KEY_RESTART as _,
    Slow = sys::KEY_SLOW as _,
    Shuffle = sys::KEY_SHUFFLE as _,
    Break = sys::KEY_BREAK as _,
    Previous = sys::KEY_PREVIOUS as _,
    Digits = sys::KEY_DIGITS as _,
    Teen = sys::KEY_TEEN as _,
    Twen = sys::KEY_TWEN as _,
    /// Media Select Video Phone
    Videophone = sys::KEY_VIDEOPHONE as _,
    /// Media Select Games
    Games = sys::KEY_GAMES as _,
    /// AC Zoom In
    ZoomIn = sys::KEY_ZOOMIN as _,
    /// AC Zoom Out
    ZoomOut = sys::KEY_ZOOMOUT as _,
    /// AC Zoom
    ZoomReset = sys::KEY_ZOOMRESET as _,
    /// AL Word Processor
    WordProcessor = sys::KEY_WORDPROCESSOR as _,
    /// AL Text Editor
    Editor = sys::KEY_EDITOR as _,
    /// AL Spreadsheet
    Spreadsheet = sys::KEY_SPREADSHEET as _,
    /// AL Graphics Editor
    GraphicsEditor = sys::KEY_GRAPHICSEDITOR as _,
    /// AL Presentation App
    Presentation = sys::KEY_PRESENTATION as _,
    /// AL Database App
    Database = sys::KEY_DATABASE as _,
    /// AL Newsreader
    News = sys::KEY_NEWS as _,
    /// AL Voicemail
    Voicemail = sys::KEY_VOICEMAIL as _,
    /// AL Contacts/Address Book
    AddressBook = sys::KEY_ADDRESSBOOK as _,
    /// AL Instant Messaging
    Messenger = sys::KEY_MESSENGER as _,
    /// Turn display (LCD) on and off (aka KeyBrightnessToggle)
    DisplayToggle = sys::KEY_DISPLAYTOGGLE as _,
    // KeyBrightnessToggle = KeyDisplayToggle
    /// AL Spell Check
    SpellCheck = sys::KEY_SPELLCHECK as _,
    /// AL Logoff
    Logoff = sys::KEY_LOGOFF as _,

    Dollar = sys::KEY_DOLLAR as _,
    Euro = sys::KEY_EURO as _,

    /// Consumer - transport controls
    FrameBack = sys::KEY_FRAMEBACK as _,
    FrameForward = sys::KEY_FRAMEFORWARD as _,
    /// GenDesc - system context menu
    ContextMenu = sys::KEY_CONTEXT_MENU as _,
    /// Consumer - transport control
    MediaRepeat = sys::KEY_MEDIA_REPEAT as _,
    /// 10 channels up (10+)
    TenChannelsUp = sys::KEY_10CHANNELSUP as _,
    /// 10 channels down (10-)
    TenChannelsDown = sys::KEY_10CHANNELSDOWN as _,
    /// AL Image Browser
    Images = sys::KEY_IMAGES as _,

    Unknown1BB,
    Unknown1BC,
    Unknown1BD,
    Unknown1BE,
    Unknown1BF,

    DelEol = sys::KEY_DEL_EOL as _,
    DelEos = sys::KEY_DEL_EOS as _,
    InsLine = sys::KEY_INS_LINE as _,
    DelLine = sys::KEY_DEL_LINE as _,

    Unknown1C4,
    Unknown1C5,
    Unknown1C6,
    Unknown1C7,
    Unknown1C8,
    Unknown1C9,
    Unknown1CA,
    Unknown1CB,
    Unknown1CC,
    Unknown1CD,
    Unknown1CE,
    Unknown1CF,

    Fn = sys::KEY_FN as _,
    FnEsc = sys::KEY_FN_ESC as _,
    FnF1 = sys::KEY_FN_F1 as _,
    FnF2 = sys::KEY_FN_F2 as _,
    FnF3 = sys::KEY_FN_F3 as _,
    FnF4 = sys::KEY_FN_F4 as _,
    FnF5 = sys::KEY_FN_F5 as _,
    FnF6 = sys::KEY_FN_F6 as _,
    FnF7 = sys::KEY_FN_F7 as _,
    FnF8 = sys::KEY_FN_F8 as _,
    FnF9 = sys::KEY_FN_F9 as _,
    FnF10 = sys::KEY_FN_F10 as _,
    FnF11 = sys::KEY_FN_F11 as _,
    FnF12 = sys::KEY_FN_F12 as _,
    Fn1 = sys::KEY_FN_1 as _,
    Fn2 = sys::KEY_FN_2 as _,
    FnD = sys::KEY_FN_D as _,
    FnE = sys::KEY_FN_E as _,
    FnF = sys::KEY_FN_F as _,
    FnS = sys::KEY_FN_S as _,
    FnB = sys::KEY_FN_B as _,

    Unknown1E5,
    Unknown1E6,
    Unknown1E7,
    Unknown1E8,
    Unknown1E9,
    Unknown1EA,
    Unknown1EB,
    Unknown1EC,
    Unknown1ED,
    Unknown1EE,
    Unknown1EF,
    Unknown1F0,

    BrlDot1 = sys::KEY_BRL_DOT1 as _,
    BrlDot2 = sys::KEY_BRL_DOT2 as _,
    BrlDot3 = sys::KEY_BRL_DOT3 as _,
    BrlDot4 = sys::KEY_BRL_DOT4 as _,
    BrlDot5 = sys::KEY_BRL_DOT5 as _,
    BrlDot6 = sys::KEY_BRL_DOT6 as _,
    BrlDot7 = sys::KEY_BRL_DOT7 as _,
    BrlDot8 = sys::KEY_BRL_DOT8 as _,
    BrlDot9 = sys::KEY_BRL_DOT9 as _,
    BrlDot10 = sys::KEY_BRL_DOT10 as _,

    Unknown1FB,
    Unknown1FC,
    Unknown1FD,
    Unknown1FE,
    Unknown1FF,

    /// used by phones, remote controls,
    Numeric0 = sys::KEY_NUMERIC_0 as _,
    /// and other keypads
    Numeric1 = sys::KEY_NUMERIC_1 as _,
    Numeric2 = sys::KEY_NUMERIC_2 as _,
    Numeric3 = sys::KEY_NUMERIC_3 as _,
    Numeric4 = sys::KEY_NUMERIC_4 as _,
    Numeric5 = sys::KEY_NUMERIC_5 as _,
    Numeric6 = sys::KEY_NUMERIC_6 as _,
    Numeric7 = sys::KEY_NUMERIC_7 as _,
    Numeric8 = sys::KEY_NUMERIC_8 as _,
    Numeric9 = sys::KEY_NUMERIC_9 as _,
    NumericStar = sys::KEY_NUMERIC_STAR as _,
    NumericPound = sys::KEY_NUMERIC_POUND as _,
    /// Phone key A - HUT Telephony 0xb9
    NumericA = sys::KEY_NUMERIC_A as _,
    NumericB = sys::KEY_NUMERIC_B as _,
    NumericC = sys::KEY_NUMERIC_C as _,
    NumericD = sys::KEY_NUMERIC_D as _,

    CameraFocus = sys::KEY_CAMERA_FOCUS as _,
    /// WiFi Protected Setup key
    WpsButton = sys::KEY_WPS_BUTTON as _,

    /// Request switch touchpad on or off
    TouchpadToggle = sys::KEY_TOUCHPAD_TOGGLE as _,
    TouchpadOn = sys::KEY_TOUCHPAD_ON as _,
    TouchpadOff = sys::KEY_TOUCHPAD_OFF as _,

    CameraZoomin = sys::KEY_CAMERA_ZOOMIN as _,
    CameraZoomout = sys::KEY_CAMERA_ZOOMOUT as _,
    CameraUp = sys::KEY_CAMERA_UP as _,
    CameraDown = sys::KEY_CAMERA_DOWN as _,
    CameraLeft = sys::KEY_CAMERA_LEFT as _,
    CameraRight = sys::KEY_CAMERA_RIGHT as _,

    AttendantOn = sys::KEY_ATTENDANT_ON as _,
    AttendantOff = sys::KEY_ATTENDANT_OFF as _,
    /// Attendant call on or off
    AttendantToggle = sys::KEY_ATTENDANT_TOGGLE as _,
    /// Reading light on or off
    LightsToggle = sys::KEY_LIGHTS_TOGGLE as _,

    Unknown21F,

    ButtonDpadUp = sys::BTN_DPAD_UP as _,
    ButtonDpadDown = sys::BTN_DPAD_DOWN as _,
    ButtonDpadLeft = sys::BTN_DPAD_LEFT as _,
    ButtonDpadRight = sys::BTN_DPAD_RIGHT as _,

    Unknown224,
    Unknown225,
    Unknown226,
    Unknown227,
    Unknown228,
    Unknown229,
    Unknown22A,
    Unknown22B,
    Unknown22C,
    Unknown22D,
    Unknown22E,
    Unknown22F,

    /// Ambient light sensor
    AlsToggle = sys::KEY_ALS_TOGGLE as _,
    RotateLockToggle = sys::KEY_ROTATE_LOCK_TOGGLE as _,

    Unknown232,
    Unknown233,
    Unknown234,
    Unknown235,
    Unknown236,
    Unknown237,
    Unknown238,
    Unknown239,
    Unknown23A,
    Unknown23B,
    Unknown23C,
    Unknown23D,
    Unknown23E,
    Unknown23F,

    /// AL Button Configuration
    ButtonConfig = sys::KEY_BUTTONCONFIG as _,
    /// AL Task/Project Manager
    TaskManager = sys::KEY_TASKMANAGER as _,
    /// AL Log/Journal/Timecard
    Journal = sys::KEY_JOURNAL as _,
    /// AL Control Panel
    ControlPanel = sys::KEY_CONTROLPANEL as _,
    /// AL Select Task/Application
    AppSelect = sys::KEY_APPSELECT as _,
    /// AL Screen Saver
    Screensaver = sys::KEY_SCREENSAVER as _,
    /// Listening Voice Command
    Voicecommand = sys::KEY_VOICECOMMAND as _,
    Assistant = sys::KEY_ASSISTANT as _,
    KbdLayoutNext = sys::KEY_KBD_LAYOUT_NEXT as _,

    Unknown249,
    Unknown24A,
    Unknown24B,
    Unknown24C,
    Unknown24D,
    Unknown24E,
    Unknown24F,

    /// Set Brightness to Minimum
    BrightnessMin = sys::KEY_BRIGHTNESS_MIN as _,
    /// Set Brightness to Maximum
    BrightnessMax = sys::KEY_BRIGHTNESS_MAX as _,

    Unknown252,
    Unknown253,
    Unknown254,
    Unknown255,
    Unknown256,
    Unknown257,
    Unknown258,
    Unknown259,
    Unknown25A,
    Unknown25B,
    Unknown25C,
    Unknown25D,
    Unknown25E,
    Unknown25F,

    InputAssistPrev = sys::KEY_KBDINPUTASSIST_PREV as _,
    InputAssistNext = sys::KEY_KBDINPUTASSIST_NEXT as _,
    InputAssistPrevGroup = sys::KEY_KBDINPUTASSIST_PREVGROUP as _,
    InputAssistNextGroup = sys::KEY_KBDINPUTASSIST_NEXTGROUP as _,
    InputAssistAccept = sys::KEY_KBDINPUTASSIST_ACCEPT as _,
    InputAssistCancel = sys::KEY_KBDINPUTASSIST_CANCEL as _,

    /// Diagonal movement keys
    RightUp = sys::KEY_RIGHT_UP as _,
    RightDown = sys::KEY_RIGHT_DOWN as _,
    LeftUp = sys::KEY_LEFT_UP as _,
    LeftDown = sys::KEY_LEFT_DOWN as _,

    /// Show Device's Root Menu
    RootMenu = sys::KEY_ROOT_MENU as _,
    /// Show Top Menu of the Media (e.g. DVD)
    MediaTopMenu = sys::KEY_MEDIA_TOP_MENU as _,
    Numeric11 = sys::KEY_NUMERIC_11 as _,
    Numeric12 = sys::KEY_NUMERIC_12 as _,

    /// Toggle Audio Description: refers to an audio service that helps blind and
    /// visually impaired consumers understand the action in a program. Note: in
    /// some countries this is referred to as "Video Description".

    AudioDesc = sys::KEY_AUDIO_DESC as _,
    Audio3dMode = sys::KEY_3D_MODE as _,
    NextFavorite = sys::KEY_NEXT_FAVORITE as _,
    StopRecord = sys::KEY_STOP_RECORD as _,
    PauseRecord = sys::KEY_PAUSE_RECORD as _,
    /// Video on Demand
    Vod = sys::KEY_VOD as _,
    Unmute = sys::KEY_UNMUTE as _,
    FastReverse = sys::KEY_FASTREVERSE as _,
    SlowReverse = sys::KEY_SLOWREVERSE as _,

    /// Control a data application associated with the currently viewed channel,
    /// e.g. teletext or data broadcast application (MHEG, MHP, HbbTV, etc.)

    Data = sys::KEY_DATA as _,
    OnscreenKeyboard = sys::KEY_ONSCREEN_KEYBOARD as _,
    PrivacyScreenToggle = sys::KEY_PRIVACY_SCREEN_TOGGLE as _,
    SelectiveScreenshot = sys::KEY_SELECTIVE_SCREENSHOT as _,

    Unknown27B,
    Unknown27C,
    Unknown27D,
    Unknown27E,
    Unknown27F,

    Unknown280,
    Unknown281,
    Unknown282,
    Unknown283,
    Unknown284,
    Unknown285,
    Unknown286,
    Unknown287,
    Unknown288,
    Unknown289,
    Unknown28A,
    Unknown28B,
    Unknown28C,
    Unknown28D,
    Unknown28E,
    Unknown28F,

    Macro1 = sys::KEY_MACRO1 as _,
    Macro2 = sys::KEY_MACRO2 as _,
    Macro3 = sys::KEY_MACRO3 as _,
    Macro4 = sys::KEY_MACRO4 as _,
    Macro5 = sys::KEY_MACRO5 as _,
    Macro6 = sys::KEY_MACRO6 as _,
    Macro7 = sys::KEY_MACRO7 as _,
    Macro8 = sys::KEY_MACRO8 as _,
    Macro9 = sys::KEY_MACRO9 as _,
    Macro10 = sys::KEY_MACRO10 as _,
    Macro11 = sys::KEY_MACRO11 as _,
    Macro12 = sys::KEY_MACRO12 as _,
    Macro13 = sys::KEY_MACRO13 as _,
    Macro14 = sys::KEY_MACRO14 as _,
    Macro15 = sys::KEY_MACRO15 as _,
    Macro16 = sys::KEY_MACRO16 as _,
    Macro17 = sys::KEY_MACRO17 as _,
    Macro18 = sys::KEY_MACRO18 as _,
    Macro19 = sys::KEY_MACRO19 as _,
    Macro20 = sys::KEY_MACRO20 as _,
    Macro21 = sys::KEY_MACRO21 as _,
    Macro22 = sys::KEY_MACRO22 as _,
    Macro23 = sys::KEY_MACRO23 as _,
    Macro24 = sys::KEY_MACRO24 as _,
    Macro25 = sys::KEY_MACRO25 as _,
    Macro26 = sys::KEY_MACRO26 as _,
    Macro27 = sys::KEY_MACRO27 as _,
    Macro28 = sys::KEY_MACRO28 as _,
    Macro29 = sys::KEY_MACRO29 as _,
    Macro30 = sys::KEY_MACRO30 as _,

    Unknown2AE,
    Unknown2AF,

    MacroRecordStart = sys::KEY_MACRO_RECORD_START as _,
    MacroRecordStop = sys::KEY_MACRO_RECORD_STOP as _,
    MacroPresetCycle = sys::KEY_MACRO_PRESET_CYCLE as _,
    MacroPreset1 = sys::KEY_MACRO_PRESET1 as _,
    MacroPreset2 = sys::KEY_MACRO_PRESET2 as _,
    MacroPreset3 = sys::KEY_MACRO_PRESET3 as _,

    Unknown2B6,
    Unknown2B7,

    KbdLcdMenu1 = sys::KEY_KBD_LCD_MENU1 as _,
    KbdLcdMenu2 = sys::KEY_KBD_LCD_MENU2 as _,
    KbdLcdMenu3 = sys::KEY_KBD_LCD_MENU3 as _,
    KbdLcdMenu4 = sys::KEY_KBD_LCD_MENU4 as _,
    KbdLcdMenu5 = sys::KEY_KBD_LCD_MENU5 as _,

    Unknown2BD,
    Unknown2BE,
    Unknown2BF,

    ButtonTriggerHappy1 = sys::BTN_TRIGGER_HAPPY1 as _,
    ButtonTriggerHappy2 = sys::BTN_TRIGGER_HAPPY2 as _,
    ButtonTriggerHappy3 = sys::BTN_TRIGGER_HAPPY3 as _,
    ButtonTriggerHappy4 = sys::BTN_TRIGGER_HAPPY4 as _,
    ButtonTriggerHappy5 = sys::BTN_TRIGGER_HAPPY5 as _,
    ButtonTriggerHappy6 = sys::BTN_TRIGGER_HAPPY6 as _,
    ButtonTriggerHappy7 = sys::BTN_TRIGGER_HAPPY7 as _,
    ButtonTriggerHappy8 = sys::BTN_TRIGGER_HAPPY8 as _,
    ButtonTriggerHappy9 = sys::BTN_TRIGGER_HAPPY9 as _,
    ButtonTriggerHappy10 = sys::BTN_TRIGGER_HAPPY10 as _,
    ButtonTriggerHappy11 = sys::BTN_TRIGGER_HAPPY11 as _,
    ButtonTriggerHappy12 = sys::BTN_TRIGGER_HAPPY12 as _,
    ButtonTriggerHappy13 = sys::BTN_TRIGGER_HAPPY13 as _,
    ButtonTriggerHappy14 = sys::BTN_TRIGGER_HAPPY14 as _,
    ButtonTriggerHappy15 = sys::BTN_TRIGGER_HAPPY15 as _,
    ButtonTriggerHappy16 = sys::BTN_TRIGGER_HAPPY16 as _,
    ButtonTriggerHappy17 = sys::BTN_TRIGGER_HAPPY17 as _,
    ButtonTriggerHappy18 = sys::BTN_TRIGGER_HAPPY18 as _,
    ButtonTriggerHappy19 = sys::BTN_TRIGGER_HAPPY19 as _,
    ButtonTriggerHappy20 = sys::BTN_TRIGGER_HAPPY20 as _,
    ButtonTriggerHappy21 = sys::BTN_TRIGGER_HAPPY21 as _,
    ButtonTriggerHappy22 = sys::BTN_TRIGGER_HAPPY22 as _,
    ButtonTriggerHappy23 = sys::BTN_TRIGGER_HAPPY23 as _,
    ButtonTriggerHappy24 = sys::BTN_TRIGGER_HAPPY24 as _,
    ButtonTriggerHappy25 = sys::BTN_TRIGGER_HAPPY25 as _,
    ButtonTriggerHappy26 = sys::BTN_TRIGGER_HAPPY26 as _,
    ButtonTriggerHappy27 = sys::BTN_TRIGGER_HAPPY27 as _,
    ButtonTriggerHappy28 = sys::BTN_TRIGGER_HAPPY28 as _,
    ButtonTriggerHappy29 = sys::BTN_TRIGGER_HAPPY29 as _,
    ButtonTriggerHappy30 = sys::BTN_TRIGGER_HAPPY30 as _,
    ButtonTriggerHappy31 = sys::BTN_TRIGGER_HAPPY31 as _,
    ButtonTriggerHappy32 = sys::BTN_TRIGGER_HAPPY32 as _,
    ButtonTriggerHappy33 = sys::BTN_TRIGGER_HAPPY33 as _,
    ButtonTriggerHappy34 = sys::BTN_TRIGGER_HAPPY34 as _,
    ButtonTriggerHappy35 = sys::BTN_TRIGGER_HAPPY35 as _,
    ButtonTriggerHappy36 = sys::BTN_TRIGGER_HAPPY36 as _,
    ButtonTriggerHappy37 = sys::BTN_TRIGGER_HAPPY37 as _,
    ButtonTriggerHappy38 = sys::BTN_TRIGGER_HAPPY38 as _,
    ButtonTriggerHappy39 = sys::BTN_TRIGGER_HAPPY39 as _,
    ButtonTriggerHappy40 = sys::BTN_TRIGGER_HAPPY40 as _,

    Unknown2E8,
    Unknown2E9,
    Unknown2EA,
    Unknown2EB,
    Unknown2EC,
    Unknown2ED,
    Unknown2EE,
    Unknown2EF,

    Unknown2F0,
    Unknown2F1,
    Unknown2F2,
    Unknown2F3,
    Unknown2F4,
    Unknown2F5,
    Unknown2F6,
    Unknown2F7,
    Unknown2F8,
    Unknown2F9,
    Unknown2FA,
    Unknown2FB,
    Unknown2FC,
    Unknown2FD,
    Unknown2FE,
    Unknown2FF,
}

impl_iterable! { Key(0, sys::KEY_CNT) }

impl Key {
    /// Determines whether the given value represents a button.
    ///
    /// Buttons are often found on a mouse or gamepad.
    pub const fn is_button(&self) -> bool {
        let key = *self as u16;
        ((key >= Key::Button0 as _) & (key < Key::Ok as _)) | (key >= Key::ButtonTriggerHappy1 as _)
    }

    /// Determines whether the given value represents a keyboard key.
    pub const fn is_key(&self) -> bool {
        !self.is_button()
    }
}
