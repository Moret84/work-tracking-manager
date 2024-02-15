use serde::{Serialize, Deserialize};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops;
use std::str::FromStr;

pub struct WorkDuration {
    pub days: u16,
    pub hours: u16,
    pub minutes: u16
}

const SEMI_COLON: char = ':';

impl ops::AddAssign<WorkDuration> for WorkDuration {
    fn add_assign(&mut self, rhs: WorkDuration) {
        self.days += rhs.days;
        self.hours += rhs.hours;
        self.minutes += rhs.minutes;

        self.hours += self.minutes / 60;
        self.minutes %= 60;
        self.days += self.hours / 24;
        self.hours %= 24;
    }
}

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
}

impl Display for WorkDuration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut str = String::new();

        Self::append_with_sep_if_not_empty(&mut str, self.days.into());
        str.push_str(&format!("{}:", self.hours));
        str.push_str(&format!("{:02}", self.minutes));

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

impl Serialize for WorkDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for WorkDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let serialized_data = String::deserialize(deserializer)?;
            let work_duration = WorkDuration::from_str(&serialized_data).map_err(serde::de::Error::custom)?;
            Ok(work_duration)
    }
}
