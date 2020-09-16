use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        println!("new: hours: {}, min: {}", hours, minutes);
        let temp_min = hours * 60 + minutes;
        let temp_clock = Clock{
            hours: 0,
            minutes: 0,
        };
        temp_clock.add_minutes(temp_min)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("add: clock:{}, min: {}", self, minutes);

        let mut temp_min = self.minutes + self.hours + minutes;
        println!("add minutes:{}", temp_min);
        let mut temp_hours = 0;
        let is_negative = if temp_min < 0 { true } else { false };
        if is_negative {
            temp_min = temp_min * -1;
        }
        
        temp_hours = ((temp_min - (temp_min % 60)) / 60) % 24;
        temp_min = temp_min % 60;
        
        if is_negative {
            temp_hours = temp_hours * -1;
            temp_min = temp_min * -1;
        }

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
