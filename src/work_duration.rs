use std::str::FromStr;

use std::fmt;
use std::fmt::{Display, Formatter};

pub struct WorkDuration {
    pub days: u64,
    pub hours: u8,
    pub minutes: u8
}

const SEMI_COLON: char = ':';

impl WorkDuration {
    fn new() -> WorkDuration {
        WorkDuration {
            days: 0,
            hours: 0,
            minutes: 0
        }
    }


    fn append_with_sep_if_not_empty(str: &mut String, to_append: u64) -> &String {
        if to_append == 0 {
            return str;
        }

       str.push_str(&format!("{}:", to_append));

       return str;
    }

    fn append_if_not_empty(str: &mut String, to_append: u64) -> &String {
        if to_append == 0 {
            return str;
        }

       str.push_str(&format!("{}", to_append));

       return str;
    }
}
impl Display for WorkDuration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut str = String::new();

        Self::append_with_sep_if_not_empty(&mut str, self.days);
        Self::append_with_sep_if_not_empty(&mut str, self.hours.into());
        Self::append_if_not_empty(&mut str, self.minutes.into());

        return write!(f, "{}", str);
    }
}

impl FromStr for WorkDuration {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut work_duration = WorkDuration::new();
        let mut part_string = String::new();
        let mut element = 0;

        for c in value.chars().rev() {
            if c != SEMI_COLON {
                part_string.insert(0, c);
                continue;
            }

            match element {
                0 => work_duration.minutes = part_string.parse().unwrap(),
                1 => work_duration.hours = part_string.parse().unwrap(),
                2 => work_duration.days = part_string.parse().unwrap(),
                _ => return Err("error parsing work duration".to_string())
            }
            element += 1;
            part_string.clear();
        }

        if part_string.is_empty() {
            return Ok(work_duration);
        }

        match element {
            0 => work_duration.minutes = part_string.parse().unwrap(),
            1 => work_duration.hours = part_string.parse().unwrap(),
            2 => work_duration.days = part_string.parse().unwrap(),
            _ => return Err("error parsing work duration".to_string())
        }

        return Ok(work_duration);
    }
}
