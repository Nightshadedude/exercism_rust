use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let temp_min = hours * 60 + minutes;
        let temp_clock = Clock{
            hours: 0,
            minutes: 0,
        };
        temp_clock.add_minutes(temp_min)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut temp_min = self.minutes + ( self.hours * 60 ) + minutes;
        while temp_min < 0 {
            temp_min = temp_min + ( 60 * 24)
        }
        temp_min = temp_min % ( 60 * 24);
        let temp_hours = ((temp_min - (temp_min % 60)) / 60) % 24;
        temp_min = temp_min % 60;

        Clock{
            hours: temp_hours,
            minutes: temp_min,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
