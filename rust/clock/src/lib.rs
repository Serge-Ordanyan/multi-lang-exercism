use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // total minutes since 00:00
        let total_minutes = hours * 60 + minutes;
        let total_minutes = ((total_minutes % 1440) + 1440) % 1440; // normalize to 0..1439
        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

// Implement Display so we can use to_string()
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
