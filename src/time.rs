use std::{fmt, cmp};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use sys::timeval;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(feature = "with-serde", derive(Deserialize, Serialize))]
pub struct EventTime(
#[cfg_attr(feature = "with-serde", serde(with = "TimevalDef"))]
timeval
);

#[cfg(feature = "with-serde")]
#[derive(Serialize, Deserialize)]
#[serde(remote = "timeval")]
#[allow(dead_code)]
struct TimevalDef {
    tv_sec: i64,
    tv_usec: i64,
}

impl EventTime {
    pub fn new(secs: i64, usecs: i64) -> Self {
        EventTime(timeval {
            tv_sec: secs,
            tv_usec: usecs,
        })
    }

    pub fn from_timeval(time: timeval) -> Self {
        EventTime(time)
    }

    pub fn seconds(&self) -> i64 {
        (self.0).tv_sec
    }

    pub fn set_seconds(&mut self, value: i64) {
        (self.0).tv_sec = value
    }

    pub fn microseconds(&self) -> i64 {
        (self.0).tv_usec
    }

    pub fn set_microseconds(&mut self, value: i64) {
        (self.0).tv_usec = value
    }

    pub fn into_inner(self) -> timeval {
        self.0
    }
}

impl Default for EventTime {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl fmt::Debug for EventTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EventTime")
            .field("seconds", &(self.0).tv_sec)
            .field("microseconds", &(self.0).tv_usec)
            .finish()
    }
}

impl From<timeval> for EventTime {
    fn from(time: timeval) -> Self {
        EventTime::from_timeval(time)
    }
}

impl From<EventTime> for timeval {
    fn from(time: EventTime) -> Self {
        time.into_inner()
    }
}

impl Deref for EventTime {
    type Target = timeval;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EventTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<timeval> for EventTime {
    fn as_ref(&self) -> &timeval {
        &self.0
    }
}

impl<'a> From<&'a timeval> for &'a EventTime {
    fn from(time: &'a timeval) -> Self {
        unsafe {
            let raw = time as *const _ as *const _;
            &*raw
        }
    }
}

impl<'a> From<&'a mut timeval> for &'a mut EventTime {
    fn from(time: &'a mut timeval) -> Self {
        unsafe {
            let raw = time as *mut _ as *mut _;
            &mut *raw
        }
    }
}

impl PartialOrd for EventTime {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EventTime {
    fn eq(&self, other: &Self) -> bool {
        (self.0).tv_sec == (other.0).tv_sec &&
            (self.0).tv_usec == (other.0).tv_usec
    }
}

impl Hash for EventTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0).tv_sec.hash(state);
        (self.0).tv_usec.hash(state);
    }
}

impl Ord for EventTime {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.0).tv_sec.cmp(&(other.0).tv_sec)
            .then((self.0).tv_usec.cmp(&(other.0).tv_usec))
    }
}

impl Eq for EventTime { }
