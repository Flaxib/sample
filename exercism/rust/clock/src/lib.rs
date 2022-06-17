use std::cmp::PartialEq;
use std::fmt;

pub struct Clock(i32, i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_from_minutes = minutes.div_euclid(60);
        let normalised_minutes = minutes.rem_euclid(60);
        let normalised_hours = (hours + hours_from_minutes).rem_euclid(24);
        Clock {
            0: normalised_hours,
            1: normalised_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.0, self.1 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.0, self.1)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.0 == other.0 && self.1 == other.1
    }

    fn ne(&self, other: &Clock) -> bool {
        !self.eq(other)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock")
            .field("hours", &self.0)
            .field("minutes", &self.1)
            .finish()
    }
}
