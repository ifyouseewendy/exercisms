use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Clock { hours, minutes };
        clock.consolidate();
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn consolidate(&mut self) {
        let mut h = 0;
        let mut m = self.minutes;
        while m < 0   { m += 60; h -= 1; }
        while m >= 60 { m -= 60; h += 1; }
        self.minutes = m;

        h += self.hours;
        while h < 0   { h += 24; }
        while h >= 24 { h -= 24; }
        self.hours = h;
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:0>2}:{:0>2}", self.hours, self.minutes))
    }
}
