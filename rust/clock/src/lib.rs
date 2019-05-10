use std::fmt;
use std::i32;
use std::f64;

#[derive(Debug, Eq)]
pub struct Clock {
    seconds: i32
}

impl Clock {

    const MAX_SECONDS_IN_DAY:i32 = 86400i32;

    fn calc_string(&self) -> String {
        let hours = ((self.seconds / 3600) as f64).floor();
        let minutes = (((self.seconds / 60 ) as f64).floor() as i32 )% 60;
        format!("{:02}:{:02}", hours, minutes)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {

        let seconds_new = hours*60*60 + minutes*60;
        match seconds_new % Clock::MAX_SECONDS_IN_DAY {
            n if n < 0 => Clock{ seconds: Clock::MAX_SECONDS_IN_DAY+n }, // n is negative, so count-off those seconds from max secs
            n => Clock{ seconds: n }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        match (self.seconds + minutes*60) % Clock::MAX_SECONDS_IN_DAY {
            n if n < 0 => Clock{ seconds: Clock::MAX_SECONDS_IN_DAY+n }, // n is negative, so count-off those seconds from max secs
            n => Clock{ seconds: n }
        }
    }

    pub fn to_string(&self) -> String {
        self.calc_string()
    }


}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.seconds == other.seconds
    }
}

