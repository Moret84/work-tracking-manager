use serde::{Serialize, Deserialize};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct WorkDuration {
    pub minutes: u32
}

const SEMI_COLON: char = ':';
const MINUTES_IN_HOUR: u32 = 60;
const MINUTES_IN_DAY: u32 = 450;

impl ops::AddAssign<WorkDuration> for WorkDuration {
    fn add_assign(&mut self, rhs: WorkDuration) {
        self.minutes += rhs.minutes;
    }
}

impl WorkDuration {
    fn new() -> WorkDuration {
        WorkDuration {
            minutes: 0
        }
    }

    fn append_with_sep_if_not_empty(str: &mut String, to_append: u64) -> &String {
        if to_append == 0 {
            return str;
        }

       str.push_str(&format!("{:02}:", to_append));

       return str;
    }
}

impl Display for WorkDuration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut str = String::new();

        let days = self.minutes / MINUTES_IN_DAY;
        let mut spare_minutes = self.minutes % MINUTES_IN_DAY;
        let hours = spare_minutes / MINUTES_IN_HOUR;
        spare_minutes = spare_minutes % 60;

        Self::append_with_sep_if_not_empty(&mut str, days.into());
        str.push_str(&format!("{:02}:", hours));
        str.push_str(&format!("{:02}", spare_minutes));

        return write!(f, "{} total: {:.2}", str, self.minutes as f64 / MINUTES_IN_DAY as f64);
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
                1 => work_duration.minutes += part_string.parse::<u32>().unwrap() * MINUTES_IN_HOUR,
                2 => work_duration.minutes += part_string.parse::<u32>().unwrap() * MINUTES_IN_DAY,
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
            1 => work_duration.minutes += part_string.parse::<u32>().unwrap() * MINUTES_IN_HOUR,
            2 => work_duration.minutes += part_string.parse::<u32>().unwrap() * MINUTES_IN_DAY,
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
