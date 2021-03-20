use chrono::{Duration, NaiveTime, Timelike};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    time: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = (hours + (minutes / 60)) % 24;
        let mut m = minutes % 60;
        if h < 0 {
            h += 24;
        }
        if m < 0 {
            h -= 1;
            m += 60;
        }
        Clock {
            time: NaiveTime::from_hms(h as u32, m as u32, 0),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            time: self.time + Duration::minutes(minutes as i64),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.time.hour(), self.time.minute())
    }
}
