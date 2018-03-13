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
pub enum Key {
    KeyReserved = sys::KEY_RESERVED as _,
    KeyEsc = sys::KEY_ESC as _,
    Key1 = sys::KEY_1 as _,
    Key2 = sys::KEY_2 as _,
    Key3 = sys::KEY_3 as _,
    Key4 = sys::KEY_4 as _,
    Key5 = sys::KEY_5 as _,
    Key6 = sys::KEY_6 as _,
    Key7 = sys::KEY_7 as _,
    Key8 = sys::KEY_8 as _,
    Key9 = sys::KEY_9 as _,
    Key0 = sys::KEY_0 as _,
    KeyMinus = sys::KEY_MINUS as _,
    KeyEqual = sys::KEY_EQUAL as _,
    KeyBackspace = sys::KEY_BACKSPACE as _,
    KeyTab = sys::KEY_TAB as _,
    KeyQ = sys::KEY_Q as _,
    KeyW = sys::KEY_W as _,
    KeyE = sys::KEY_E as _,
    KeyR = sys::KEY_R as _,
    KeyT = sys::KEY_T as _,
    KeyY = sys::KEY_Y as _,
    KeyU = sys::KEY_U as _,
    KeyI = sys::KEY_I as _,
    KeyO = sys::KEY_O as _,
    KeyP = sys::KEY_P as _,
    KeyLeftBrace = sys::KEY_LEFTBRACE as _,
    KeyRightBrace = sys::KEY_RIGHTBRACE as _,
    KeyEnter = sys::KEY_ENTER as _,
    KeyLeftCtrl = sys::KEY_LEFTCTRL as _,
    KeyA = sys::KEY_A as _,
    KeyS = sys::KEY_S as _,
    KeyD = sys::KEY_D as _,
    KeyF = sys::KEY_F as _,
    KeyG = sys::KEY_G as _,
    KeyH = sys::KEY_H as _,
    KeyJ = sys::KEY_J as _,
    KeyK = sys::KEY_K as _,
    KeyL = sys::KEY_L as _,
    KeySemicolon = sys::KEY_SEMICOLON as _,
    KeyApostrophe = sys::KEY_APOSTROPHE as _,
    KeyGrave = sys::KEY_GRAVE as _,
    KeyLeftShift = sys::KEY_LEFTSHIFT as _,
    KeyBackslash = sys::KEY_BACKSLASH as _,
    KeyZ = sys::KEY_Z as _,
    KeyX = sys::KEY_X as _,
    KeyC = sys::KEY_C as _,
    KeyV = sys::KEY_V as _,
    KeyB = sys::KEY_B as _,
    KeyN = sys::KEY_N as _,
    KeyM = sys::KEY_M as _,
    KeyComma = sys::KEY_COMMA as _,
    KeyDot = sys::KEY_DOT as _,
    KeySlash = sys::KEY_SLASH as _,
    KeyRightShift = sys::KEY_RIGHTSHIFT as _,
    KeyKpAsterisk = sys::KEY_KPASTERISK as _,
    KeyLeftAlt = sys::KEY_LEFTALT as _,
    KeySpace = sys::KEY_SPACE as _,
    KeyCapsLock = sys::KEY_CAPSLOCK as _,
    KeyF1 = sys::KEY_F1 as _,
    KeyF2 = sys::KEY_F2 as _,
    KeyF3 = sys::KEY_F3 as _,
    KeyF4 = sys::KEY_F4 as _,
    KeyF5 = sys::KEY_F5 as _,
    KeyF6 = sys::KEY_F6 as _,
    KeyF7 = sys::KEY_F7 as _,
    KeyF8 = sys::KEY_F8 as _,
    KeyF9 = sys::KEY_F9 as _,
    KeyF10 = sys::KEY_F10 as _,
    KeyNumLock = sys::KEY_NUMLOCK as _,
    KeyScrollLock = sys::KEY_SCROLLLOCK as _,
    KeyKp7 = sys::KEY_KP7 as _,
    KeyKp8 = sys::KEY_KP8 as _,
    KeyKp9 = sys::KEY_KP9 as _,
    KeyKpMinus = sys::KEY_KPMINUS as _,
    KeyKp4 = sys::KEY_KP4 as _,
    KeyKp5 = sys::KEY_KP5 as _,
    KeyKp6 = sys::KEY_KP6 as _,
    KeyKpPlus = sys::KEY_KPPLUS as _,
    KeyKp1 = sys::KEY_KP1 as _,
    KeyKp2 = sys::KEY_KP2 as _,
    KeyKp3 = sys::KEY_KP3 as _,
    KeyKp0 = sys::KEY_KP0 as _,
    KeyKpDot = sys::KEY_KPDOT as _,

    Unknown54,

    KeyZenkakuHankaku = sys::KEY_ZENKAKUHANKAKU as _,
    Key102nd = sys::KEY_102ND as _,
    KeyF11 = sys::KEY_F11 as _,
    KeyF12 = sys::KEY_F12 as _,
    KeyRo = sys::KEY_RO as _,
    KeyKatakana = sys::KEY_KATAKANA as _,
    KeyHiragana = sys::KEY_HIRAGANA as _,
    KeyHenkan = sys::KEY_HENKAN as _,
    KeyKatakanaHiragana = sys::KEY_KATAKANAHIRAGANA as _,
    KeyMuhenkan = sys::KEY_MUHENKAN as _,
    KeyKpJpComma = sys::KEY_KPJPCOMMA as _,
    KeyKpEnter = sys::KEY_KPENTER as _,
    KeyRightCtrl = sys::KEY_RIGHTCTRL as _,
    KeyKpSlash = sys::KEY_KPSLASH as _,
    KeySysrq = sys::KEY_SYSRQ as _,
    KeyRightAlt = sys::KEY_RIGHTALT as _,
    KeyLineFeed = sys::KEY_LINEFEED as _,
    KeyHome = sys::KEY_HOME as _,
    KeyUp = sys::KEY_UP as _,
    KeyPageUp = sys::KEY_PAGEUP as _,
    KeyLeft = sys::KEY_LEFT as _,
    KeyRight = sys::KEY_RIGHT as _,
    KeyEnd = sys::KEY_END as _,
    KeyDown = sys::KEY_DOWN as _,
    KeyPageDown = sys::KEY_PAGEDOWN as _,
    KeyInsert = sys::KEY_INSERT as _,
    KeyDelete = sys::KEY_DELETE as _,
    KeyMacro = sys::KEY_MACRO as _,

    KeyMute = sys::KEY_MUTE as _,
    KeyVolumeDown = sys::KEY_VOLUMEDOWN as _,
    KeyVolumeUp = sys::KEY_VOLUMEUP as _,
    /// SC System Power Down
    KeyPower = sys::KEY_POWER as _,
    KeyKpEqual = sys::KEY_KPEQUAL as _,
    KeyKpPlusMinus = sys::KEY_KPPLUSMINUS as _,
    KeyPause = sys::KEY_PAUSE as _,
    /// AL Compiz Scale (Expose)
    KeyScale = sys::KEY_SCALE as _,

    KeyKpComma = sys::KEY_KPCOMMA as _,
    /// KeyHangeul / KeyHanguel
    KeyHangul = sys::KEY_HANGEUL as _,
    // KeyHangeul = KeyHangul
    // KeyHanguel = KeyHangul
    KeyHanja = sys::KEY_HANJA as _,
    KeyYen = sys::KEY_YEN as _,
    KeyLeftMeta = sys::KEY_LEFTMETA as _,
    KeyRightMeta = sys::KEY_RIGHTMETA as _,
    KeyCompose = sys::KEY_COMPOSE as _,

    /// AC Stop
    KeyStop = sys::KEY_STOP as _,
    KeyAgain = sys::KEY_AGAIN as _,
    /// AC Properties
    KeyProps = sys::KEY_PROPS as _,
    /// AC Undo
    KeyUndo = sys::KEY_UNDO as _,
    KeyFront = sys::KEY_FRONT as _,
    /// AC Copy
    KeyCopy = sys::KEY_COPY as _,
    /// AC Open
    KeyOpen = sys::KEY_OPEN as _,
    /// AC Paste
    KeyPaste = sys::KEY_PASTE as _,
    /// AC Search
    KeyFind = sys::KEY_FIND as _,
    /// AC Cut
    KeyCut = sys::KEY_CUT as _,
    /// AL Integrated Help Center
    KeyHelp = sys::KEY_HELP as _,
    /// Menu (show menu)
    KeyMenu = sys::KEY_MENU as _,
    /// AL Calculator
    KeyCalc = sys::KEY_CALC as _,
    KeySetup = sys::KEY_SETUP as _,
    /// SC System Sleep
    KeySleep = sys::KEY_SLEEP as _,
    /// System Wake Up
    KeyWakeup = sys::KEY_WAKEUP as _,
    /// AL Local Machine Browser
    KeyFile = sys::KEY_FILE as _,
    KeySendFile = sys::KEY_SENDFILE as _,
    KeyDeleteFile = sys::KEY_DELETEFILE as _,
    KeyXfer = sys::KEY_XFER as _,
    KeyProg1 = sys::KEY_PROG1 as _,
    KeyProg2 = sys::KEY_PROG2 as _,
    /// AL Internet Browser
    KeyWWW = sys::KEY_WWW as _,
    KeyMSDOS = sys::KEY_MSDOS as _,
    /// AL Terminal Lock/Screensaver
    /// KeyScreenLock
    KeyCoffee = sys::KEY_COFFEE as _,
    // KeyScreenLock = KeyCoffee,
    /// Display orientation for e.g. tablets (aka KeyDirectionKey)
    KeyRotateDisplay = sys::KEY_ROTATE_DISPLAY as _,
    // KeyDirectionKey = KeyRotateDisplay
    KeyCycleWindows = sys::KEY_CYCLEWINDOWS as _,
    KeyMail = sys::KEY_MAIL as _,
    /// AC Bookmarks
    KeyBookmarks = sys::KEY_BOOKMARKS as _,
    KeyComputer = sys::KEY_COMPUTER as _,
    /// AC Back
    KeyBack = sys::KEY_BACK as _,
    /// AC Forward
    KeyForward = sys::KEY_FORWARD as _,
    KeyCloseCD = sys::KEY_CLOSECD as _,
    KeyEjectCD = sys::KEY_EJECTCD as _,
    KeyEjectCloseCD = sys::KEY_EJECTCLOSECD as _,
    KeyNextSong = sys::KEY_NEXTSONG as _,
    KeyPlayPause = sys::KEY_PLAYPAUSE as _,
    KeyPreviousSong = sys::KEY_PREVIOUSSONG as _,
    KeyStopCD = sys::KEY_STOPCD as _,
    KeyRecord = sys::KEY_RECORD as _,
    KeyRewind = sys::KEY_REWIND as _,
    /// Media Select Telephone
    KeyPhone = sys::KEY_PHONE as _,
    KeyIso = sys::KEY_ISO as _,
    /// AL Consumer Control Configuration
    KeyConfig = sys::KEY_CONFIG as _,
    /// AC Home
    KeyHomepage = sys::KEY_HOMEPAGE as _,
    /// AC Refresh
    KeyRefresh = sys::KEY_REFRESH as _,
    /// AC Exit
    KeyExit = sys::KEY_EXIT as _,
    KeyMove = sys::KEY_MOVE as _,
    KeyEdit = sys::KEY_EDIT as _,
    KeyScrollUp = sys::KEY_SCROLLUP as _,
    KeyScrollDown = sys::KEY_SCROLLDOWN as _,
    KeyKpLeftParen = sys::KEY_KPLEFTPAREN as _,
    KeyKpRightParen = sys::KEY_KPRIGHTPAREN as _,
    /// AC New
    KeyNew = sys::KEY_NEW as _,
    /// AC Redo/Repeat
    KeyRedo = sys::KEY_REDO as _,

    KeyF13 = sys::KEY_F13 as _,
    KeyF14 = sys::KEY_F14 as _,
    KeyF15 = sys::KEY_F15 as _,
    KeyF16 = sys::KEY_F16 as _,
    KeyF17 = sys::KEY_F17 as _,
    KeyF18 = sys::KEY_F18 as _,
    KeyF19 = sys::KEY_F19 as _,
    KeyF20 = sys::KEY_F20 as _,
    KeyF21 = sys::KEY_F21 as _,
    KeyF22 = sys::KEY_F22 as _,
    KeyF23 = sys::KEY_F23 as _,
    KeyF24 = sys::KEY_F24 as _,

    UnknownC3,
    UnknownC4,
    UnknownC5,
    UnknownC6,
    UnknownC7,

    KeyPlayCD = sys::KEY_PLAYCD as _,
    KeyPauseCD = sys::KEY_PAUSECD as _,
    KeyProg3 = sys::KEY_PROG3 as _,
    KeyProg4 = sys::KEY_PROG4 as _,
    /// AL Dashboard
    KeyDashboard = sys::KEY_DASHBOARD as _,
    KeySuspend = sys::KEY_SUSPEND as _,
    /// AC Close
    KeyClose = sys::KEY_CLOSE as _,
    KeyPlay = sys::KEY_PLAY as _,
    KeyFastForward = sys::KEY_FASTFORWARD as _,
    KeyBassBoost = sys::KEY_BASSBOOST as _,
    /// AC Print
    KeyPrint = sys::KEY_PRINT as _,
    KeyHp = sys::KEY_HP as _,
    KeyCamera = sys::KEY_CAMERA as _,
    KeySound = sys::KEY_SOUND as _,
    KeyQuestion = sys::KEY_QUESTION as _,
    KeyEmail = sys::KEY_EMAIL as _,
    KeyChat = sys::KEY_CHAT as _,
    KeySearch = sys::KEY_SEARCH as _,
    KeyConnect = sys::KEY_CONNECT as _,
    /// AL Checkbook/Finance
    KeyFinance = sys::KEY_FINANCE as _,
    KeySport = sys::KEY_SPORT as _,
    KeyShop = sys::KEY_SHOP as _,
    KeyAlterase = sys::KEY_ALTERASE as _,
    /// AC Cancel
    KeyCancel = sys::KEY_CANCEL as _,
    KeyBrightnessDown = sys::KEY_BRIGHTNESSDOWN as _,
    KeyBrightnessUp = sys::KEY_BRIGHTNESSUP as _,
    KeyMedia = sys::KEY_MEDIA as _,

    /// Cycle between available video outputs (Monitor/LCD/TV-out/etc)
    KeySwitchVideoMode = sys::KEY_SWITCHVIDEOMODE as _,
    KeyIllumToggle = sys::KEY_KBDILLUMTOGGLE as _,
    KeyIllumDown = sys::KEY_KBDILLUMDOWN as _,
    KeyIllumUp = sys::KEY_KBDILLUMUP as _,

    /// AC Send
    KeySend = sys::KEY_SEND as _,
    /// AC Reply
    KeyReply = sys::KEY_REPLY as _,
    /// AC Forward Msg
    KeyForwardMail = sys::KEY_FORWARDMAIL as _,
    /// AC Save
    KeySave = sys::KEY_SAVE as _,
    KeyDocuments = sys::KEY_DOCUMENTS as _,

    KeyBattery = sys::KEY_BATTERY as _,

    KeyBluetooth = sys::KEY_BLUETOOTH as _,
    KeyWLAN = sys::KEY_WLAN as _,
    KeyUWB = sys::KEY_UWB as _,

    KeyUnknown = sys::KEY_UNKNOWN as _,

    /// drive next video source
    KeyVideoNext = sys::KEY_VIDEO_NEXT as _,
    /// drive previous video source
    KeyVideoPrev = sys::KEY_VIDEO_PREV as _,
    /// brightness up, after max is min
    KeyBrightnessCycle = sys::KEY_BRIGHTNESS_CYCLE as _,
    /// Set Auto Brightness: manual brightness control is off, rely on ambient
    /// (aka KeyBrightnessZero)
    KeyBrightnessAuto = sys::KEY_BRIGHTNESS_AUTO as _,
    // KeyBrightnessZero = KeyBrightnessAuto
    /// display device to off state
    KeyDisplayOff = sys::KEY_DISPLAY_OFF as _,

    /// Wireless WAN (LTE, UMTS, GSM, etc.)
    /// (aka KeyWiMAX)
    KeyWWAN = sys::KEY_WWAN as _,
    // KeyWiMAX = KeyWWAN
    /// Key that controls all radios
    KeyRfkill = sys::KEY_RFKILL as _,

    /// Mute / unmute the microphone
    KeyMicMute = sys::KEY_MICMUTE as _,

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

    Unknown149,

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

    KeyOk = sys::KEY_OK as _,
    KeySelect = sys::KEY_SELECT as _,
    KeyGoto = sys::KEY_GOTO as _,
    KeyClear = sys::KEY_CLEAR as _,
    KeyPower2 = sys::KEY_POWER2 as _,
    KeyOption = sys::KEY_OPTION as _,
    /// AL OEM Features/Tips/Tutorial
    KeyInfo = sys::KEY_INFO as _,
    KeyTime = sys::KEY_TIME as _,
    KeyVendor = sys::KEY_VENDOR as _,
    KeyArchive = sys::KEY_ARCHIVE as _,
    /// Media Select Program Guide
    KeyProgram = sys::KEY_PROGRAM as _,
    KeyChannel = sys::KEY_CHANNEL as _,
    KeyFavorites = sys::KEY_FAVORITES as _,
    KeyEPG = sys::KEY_EPG as _,
    /// Media Select Home
    KeyPVR = sys::KEY_PVR as _,
    KeyMHP = sys::KEY_MHP as _,
    KeyLanguage = sys::KEY_LANGUAGE as _,
    KeyTitle = sys::KEY_TITLE as _,
    KeySubtitle = sys::KEY_SUBTITLE as _,
    KeyAngle = sys::KEY_ANGLE as _,
    KeyZoom = sys::KEY_ZOOM as _,
    KeyMode = sys::KEY_MODE as _,
    KeyKeyboard = sys::KEY_KEYBOARD as _,
    KeyScreen = sys::KEY_SCREEN as _,
    /// Media Select Computer
    KeyPC = sys::KEY_PC as _,
    /// Media Select TV
    KeyTV = sys::KEY_TV as _,
    /// Media Select Cable
    KeyTV2 = sys::KEY_TV2 as _,
    /// Media Select VCR
    KeyVCR = sys::KEY_VCR as _,
    /// VCR Plus
    KeyVCR2 = sys::KEY_VCR2 as _,
    /// Media Select Satellite
    KeySat = sys::KEY_SAT as _,
    KeySat2 = sys::KEY_SAT2 as _,
    /// Media Select CD
    KeyCD = sys::KEY_CD as _,
    /// Media Select Tape
    KeyTape = sys::KEY_TAPE as _,
    KeyRadio = sys::KEY_RADIO as _,
    /// Media Select Tuner
    KeyTuner = sys::KEY_TUNER as _,
    KeyPlayer = sys::KEY_PLAYER as _,
    KeyText = sys::KEY_TEXT as _,
    /// Media Select DVD
    KeyDvd = sys::KEY_DVD as _,
    KeyAux = sys::KEY_AUX as _,
    KeyMp3 = sys::KEY_MP3 as _,
    /// AL Audio Browser
    KeyAudio = sys::KEY_AUDIO as _,
    /// AL Movie Browser
    KeyVideo = sys::KEY_VIDEO as _,
    KeyDirectory = sys::KEY_DIRECTORY as _,
    KeyList = sys::KEY_LIST as _,
    /// Media Select Messages
    KeyMemo = sys::KEY_MEMO as _,
    KeyCalendar = sys::KEY_CALENDAR as _,
    KeyRed = sys::KEY_RED as _,
    KeyGreen = sys::KEY_GREEN as _,
    KeyYellow = sys::KEY_YELLOW as _,
    KeyBlue = sys::KEY_BLUE as _,
    /// Channel Increment
    KeyChannelUp = sys::KEY_CHANNELUP as _,
    /// Channel Decrement
    KeyChannelDown = sys::KEY_CHANNELDOWN as _,
    KeyFirst = sys::KEY_FIRST as _,
    /// Recall Last
    KeyLast = sys::KEY_LAST as _,
    KeyAb = sys::KEY_AB as _,
    KeyNext = sys::KEY_NEXT as _,
    KeyRestart = sys::KEY_RESTART as _,
    KeySlow = sys::KEY_SLOW as _,
    KeyShuffle = sys::KEY_SHUFFLE as _,
    KeyBreak = sys::KEY_BREAK as _,
    KeyPrevious = sys::KEY_PREVIOUS as _,
    KeyDigits = sys::KEY_DIGITS as _,
    KeyTeen = sys::KEY_TEEN as _,
    KeyTwen = sys::KEY_TWEN as _,
    /// Media Select Video Phone
    KeyVideophone = sys::KEY_VIDEOPHONE as _,
    /// Media Select Games
    KeyGames = sys::KEY_GAMES as _,
    /// AC Zoom In
    KeyZoomIn = sys::KEY_ZOOMIN as _,
    /// AC Zoom Out
    KeyZoomOut = sys::KEY_ZOOMOUT as _,
    /// AC Zoom
    KeyZoomReset = sys::KEY_ZOOMRESET as _,
    /// AL Word Processor
    KeyWordProcessor = sys::KEY_WORDPROCESSOR as _,
    /// AL Text Editor
    KeyEditor = sys::KEY_EDITOR as _,
    /// AL Spreadsheet
    KeySpreadsheet = sys::KEY_SPREADSHEET as _,
    /// AL Graphics Editor
    KeyGraphicsEditor = sys::KEY_GRAPHICSEDITOR as _,
    /// AL Presentation App
    KeyPresentation = sys::KEY_PRESENTATION as _,
    /// AL Database App
    KeyDatabase = sys::KEY_DATABASE as _,
    /// AL Newsreader
    KeyNews = sys::KEY_NEWS as _,
    /// AL Voicemail
    KeyVoicemail = sys::KEY_VOICEMAIL as _,
    /// AL Contacts/Address Book
    KeyAddressBook = sys::KEY_ADDRESSBOOK as _,
    /// AL Instant Messaging
    KeyMessenger = sys::KEY_MESSENGER as _,
    /// Turn display (LCD) on and off (aka KeyBrightnessToggle)
    KeyDisplayToggle = sys::KEY_DISPLAYTOGGLE as _,
    // KeyBrightnessToggle = KeyDisplayToggle
    /// AL Spell Check
    KeySpellCheck = sys::KEY_SPELLCHECK as _,
    /// AL Logoff
    KeyLogoff = sys::KEY_LOGOFF as _,

    KeyDollar = sys::KEY_DOLLAR as _,
    KeyEuro = sys::KEY_EURO as _,

    /// Consumer - transport controls
    KeyFrameBack = sys::KEY_FRAMEBACK as _,
    KeyFrameForward = sys::KEY_FRAMEFORWARD as _,
    /// GenDesc - system context menu
    KeyContextMenu = sys::KEY_CONTEXT_MENU as _,
    /// Consumer - transport control
    KeyMediaRepeat = sys::KEY_MEDIA_REPEAT as _,
    /// 10 channels up (10+)
    Key10ChannelsUp = sys::KEY_10CHANNELSUP as _,
    /// 10 channels down (10-)
    Key10ChannelsDown = sys::KEY_10CHANNELSDOWN as _,
    /// AL Image Browser
    KeyImages = sys::KEY_IMAGES as _,

    Unknown1BB,
    Unknown1BC,
    Unknown1BD,
    Unknown1BE,
    Unknown1BF,

    KeyDelEol = sys::KEY_DEL_EOL as _,
    KeyDelEos = sys::KEY_DEL_EOS as _,
    KeyInsLine = sys::KEY_INS_LINE as _,
    KeyDelLine = sys::KEY_DEL_LINE as _,

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

    KeyFn = sys::KEY_FN as _,
    KeyFnEsc = sys::KEY_FN_ESC as _,
    KeyFnF1 = sys::KEY_FN_F1 as _,
    KeyFnF2 = sys::KEY_FN_F2 as _,
    KeyFnF3 = sys::KEY_FN_F3 as _,
    KeyFnF4 = sys::KEY_FN_F4 as _,
    KeyFnF5 = sys::KEY_FN_F5 as _,
    KeyFnF6 = sys::KEY_FN_F6 as _,
    KeyFnF7 = sys::KEY_FN_F7 as _,
    KeyFnF8 = sys::KEY_FN_F8 as _,
    KeyFnF9 = sys::KEY_FN_F9 as _,
    KeyFnF10 = sys::KEY_FN_F10 as _,
    KeyFnF11 = sys::KEY_FN_F11 as _,
    KeyFnF12 = sys::KEY_FN_F12 as _,
    KeyFn1 = sys::KEY_FN_1 as _,
    KeyFn2 = sys::KEY_FN_2 as _,
    KeyFnD = sys::KEY_FN_D as _,
    KeyFnE = sys::KEY_FN_E as _,
    KeyFnF = sys::KEY_FN_F as _,
    KeyFnS = sys::KEY_FN_S as _,
    KeyFnB = sys::KEY_FN_B as _,

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

    KeyBrlDot1 = sys::KEY_BRL_DOT1 as _,
    KeyBrlDot2 = sys::KEY_BRL_DOT2 as _,
    KeyBrlDot3 = sys::KEY_BRL_DOT3 as _,
    KeyBrlDot4 = sys::KEY_BRL_DOT4 as _,
    KeyBrlDot5 = sys::KEY_BRL_DOT5 as _,
    KeyBrlDot6 = sys::KEY_BRL_DOT6 as _,
    KeyBrlDot7 = sys::KEY_BRL_DOT7 as _,
    KeyBrlDot8 = sys::KEY_BRL_DOT8 as _,
    KeyBrlDot9 = sys::KEY_BRL_DOT9 as _,
    KeyBrlDot10 = sys::KEY_BRL_DOT10 as _,

    Unknown1FB,
    Unknown1FC,
    Unknown1FD,
    Unknown1FE,
    Unknown1FF,

    /// used by phones, remote controls,
    KeyNumeric0 = sys::KEY_NUMERIC_0 as _,
    /// and other keypads
    KeyNumeric1 = sys::KEY_NUMERIC_1 as _,
    KeyNumeric2 = sys::KEY_NUMERIC_2 as _,
    KeyNumeric3 = sys::KEY_NUMERIC_3 as _,
    KeyNumeric4 = sys::KEY_NUMERIC_4 as _,
    KeyNumeric5 = sys::KEY_NUMERIC_5 as _,
    KeyNumeric6 = sys::KEY_NUMERIC_6 as _,
    KeyNumeric7 = sys::KEY_NUMERIC_7 as _,
    KeyNumeric8 = sys::KEY_NUMERIC_8 as _,
    KeyNumeric9 = sys::KEY_NUMERIC_9 as _,
    KeyNumericStar = sys::KEY_NUMERIC_STAR as _,
    KeyNumericPound = sys::KEY_NUMERIC_POUND as _,
    /// Phone key A - HUT Telephony 0xb9
    KeyNumericA = sys::KEY_NUMERIC_A as _,
    KeyNumericB = sys::KEY_NUMERIC_B as _,
    KeyNumericC = sys::KEY_NUMERIC_C as _,
    KeyNumericD = sys::KEY_NUMERIC_D as _,

    KeyCameraFocus = sys::KEY_CAMERA_FOCUS as _,
    /// WiFi Protected Setup key
    KeyWpsButton = sys::KEY_WPS_BUTTON as _,

    /// Request switch touchpad on or off
    KeyTouchpadToggle = sys::KEY_TOUCHPAD_TOGGLE as _,
    KeyTouchpadOn = sys::KEY_TOUCHPAD_ON as _,
    KeyTouchpadOff = sys::KEY_TOUCHPAD_OFF as _,

    KeyCameraZoomin = sys::KEY_CAMERA_ZOOMIN as _,
    KeyCameraZoomout = sys::KEY_CAMERA_ZOOMOUT as _,
    KeyCameraUp = sys::KEY_CAMERA_UP as _,
    KeyCameraDown = sys::KEY_CAMERA_DOWN as _,
    KeyCameraLeft = sys::KEY_CAMERA_LEFT as _,
    KeyCameraRight = sys::KEY_CAMERA_RIGHT as _,

    KeyAttendantOn = sys::KEY_ATTENDANT_ON as _,
    KeyAttendantOff = sys::KEY_ATTENDANT_OFF as _,
    /// Attendant call on or off
    KeyAttendantToggle = sys::KEY_ATTENDANT_TOGGLE as _,
    /// Reading light on or off
    KeyLightsToggle = sys::KEY_LIGHTS_TOGGLE as _,

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
    KeyAlsToggle = sys::KEY_ALS_TOGGLE as _,

    Unknown231,
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
    KeyButtonConfig = sys::KEY_BUTTONCONFIG as _,
    /// AL Task/Project Manager
    KeyTaskManager = sys::KEY_TASKMANAGER as _,
    /// AL Log/Journal/Timecard
    KeyJournal = sys::KEY_JOURNAL as _,
    /// AL Control Panel
    KeyControlPanel = sys::KEY_CONTROLPANEL as _,
    /// AL Select Task/Application
    KeyAppSelect = sys::KEY_APPSELECT as _,
    /// AL Screen Saver
    KeyScreensaver = sys::KEY_SCREENSAVER as _,
    /// Listening Voice Command
    KeyVoicecommand = sys::KEY_VOICECOMMAND as _,

    Unknown247,
    Unknown248,
    Unknown249,
    Unknown24A,
    Unknown24B,
    Unknown24C,
    Unknown24D,
    Unknown24E,
    Unknown24F,

    /// Set Brightness to Minimum
    KeyBrightnessMin = sys::KEY_BRIGHTNESS_MIN as _,
    /// Set Brightness to Maximum
    KeyBrightnessMax = sys::KEY_BRIGHTNESS_MAX as _,

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

    KeyInputAssistPrev = sys::KEY_KBDINPUTASSIST_PREV as _,
    KeyInputAssistNext = sys::KEY_KBDINPUTASSIST_NEXT as _,
    KeyInputAssistPrevGroup = sys::KEY_KBDINPUTASSIST_PREVGROUP as _,
    KeyInputAssistNextGroup = sys::KEY_KBDINPUTASSIST_NEXTGROUP as _,
    KeyInputAssistAccept = sys::KEY_KBDINPUTASSIST_ACCEPT as _,
    KeyInputAssistCancel = sys::KEY_KBDINPUTASSIST_CANCEL as _,

    /// Diagonal movement keys
    KeyRightUp = sys::KEY_RIGHT_UP as _,
    KeyRightDown = sys::KEY_RIGHT_DOWN as _,
    KeyLeftUp = sys::KEY_LEFT_UP as _,
    KeyLeftDown = sys::KEY_LEFT_DOWN as _,

    /// Show Device's Root Menu
    KeyRootMenu = sys::KEY_ROOT_MENU as _,
    /// Show Top Menu of the Media (e.g. DVD)
    KeyMediaTopMenu = sys::KEY_MEDIA_TOP_MENU as _,
    KeyNumeric11 = sys::KEY_NUMERIC_11 as _,
    KeyNumeric12 = sys::KEY_NUMERIC_12 as _,

    /// Toggle Audio Description: refers to an audio service that helps blind and
    /// visually impaired consumers understand the action in a program. Note: in
    /// some countries this is referred to as "Video Description".

    KeyAudioDesc = sys::KEY_AUDIO_DESC as _,
    Key3dMode = sys::KEY_3D_MODE as _,
    KeyNextFavorite = sys::KEY_NEXT_FAVORITE as _,
    KeyStopRecord = sys::KEY_STOP_RECORD as _,
    KeyPauseRecord = sys::KEY_PAUSE_RECORD as _,
    /// Video on Demand
    KeyVod = sys::KEY_VOD as _,
    KeyUnmute = sys::KEY_UNMUTE as _,
    KeyFastReverse = sys::KEY_FASTREVERSE as _,
    KeySlowReverse = sys::KEY_SLOWREVERSE as _,

    /// Control a data application associated with the currently viewed channel,
    /// e.g. teletext or data broadcast application (MHEG, MHP, HbbTV, etc.)

    KeyData = sys::KEY_DATA as _,
    KeyOnscreenKeyboard = sys::KEY_ONSCREEN_KEYBOARD as _,

    Unknown279,
    Unknown27A,
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

    Unknown290,
    Unknown291,
    Unknown292,
    Unknown293,
    Unknown294,
    Unknown295,
    Unknown296,
    Unknown297,
    Unknown298,
    Unknown299,
    Unknown29A,
    Unknown29B,
    Unknown29C,
    Unknown29D,
    Unknown29E,
    Unknown29F,

    Unknown2A0,
    Unknown2A1,
    Unknown2A2,
    Unknown2A3,
    Unknown2A4,
    Unknown2A5,
    Unknown2A6,
    Unknown2A7,
    Unknown2A8,
    Unknown2A9,
    Unknown2AA,
    Unknown2AB,
    Unknown2AC,
    Unknown2AD,
    Unknown2AE,
    Unknown2AF,

    Unknown2B0,
    Unknown2B1,
    Unknown2B2,
    Unknown2B3,
    Unknown2B4,
    Unknown2B5,
    Unknown2B6,
    Unknown2B7,
    Unknown2B8,
    Unknown2B9,
    Unknown2BA,
    Unknown2BB,
    Unknown2BC,
    Unknown2BD,
    Unknown2BE,
    Unknown2BF,

    //ButtonTriggerHappy = sys::BTN_TRIGGER_HAPPY as _,
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
    pub fn is_button(&self) -> bool {
        let key = *self as u16;
        ((key >= Key::Button0 as _) && (key < Key::KeyOk as _)) || key >= Key::ButtonTriggerHappy1 as _
    }

    pub fn is_key(&self) -> bool {
        !self.is_button()
    }
}
