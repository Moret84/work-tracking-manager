use serde::{Serialize, Deserialize};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Copy, Clone)]
pub struct WorkDuration {
    pub minutes: u32
}

const SEMI_COLON: char = ':';
const MINUTES_IN_HOUR: u32 = 60;
pub const MINUTES_IN_DAY: u32 = 450;

static INCLUDE_TOTAL: AtomicBool = AtomicBool::new(false);

pub fn set_include_total(value: bool) {
    INCLUDE_TOTAL.store(value, Ordering::Relaxed);
}

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

       str
    }
}

impl Display for WorkDuration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut str = String::new();

        let days = self.minutes / MINUTES_IN_DAY;
        let mut spare_minutes = self.minutes % MINUTES_IN_DAY;
        let hours = spare_minutes / MINUTES_IN_HOUR;
        spare_minutes %= 60;

        Self::append_with_sep_if_not_empty(&mut str, days.into());
        str.push_str(&format!("{:02}:", hours));
        str.push_str(&format!("{:02}", spare_minutes));

        let mut result = str.to_string();

        if INCLUDE_TOTAL.load(Ordering::Relaxed) {
            result.push_str(
                &format!(" total: {:.2}", self.minutes as f64 / MINUTES_IN_DAY as f64)
            )}

        write!(f, "{}", result)
    }
}

fn add_duration_part(duration: &mut WorkDuration, part: &str, element: i32, original: &str) -> Result<(), String> {
    let value: u32 = part.parse()
        .map_err(|_| format!("durée invalide « {original} » : « {part} » n'est pas un nombre"))?;

    match element {
        0 => duration.minutes += value,
        1 => duration.minutes += value * MINUTES_IN_HOUR,
        2 => duration.minutes += value * MINUTES_IN_DAY,
        _ => return Err(format!("durée invalide « {original} » : trop de segments (max heures:minutes:secondes)")),
    }

    Ok(())
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

            add_duration_part(&mut work_duration, &part_string, element, value)?;

            element += 1;
            part_string.clear();
        }

        if part_string.is_empty() {
            return Ok(work_duration);
        }

        add_duration_part(&mut work_duration, &part_string, element, value)?;

        Ok(work_duration)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hours_and_minutes() {
        assert_eq!("2:30".parse::<WorkDuration>().unwrap().minutes, 150);
    }

    #[test]
    fn parses_minutes_only() {
        assert_eq!("45".parse::<WorkDuration>().unwrap().minutes, 45);
    }

    #[test]
    fn parses_days_hours_and_minutes() {
        assert_eq!("1:2:30".parse::<WorkDuration>().unwrap().minutes, MINUTES_IN_DAY + 2 * MINUTES_IN_HOUR + 30);
    }

    #[test]
    fn rejects_a_non_numeric_segment() {
        assert!("abc".parse::<WorkDuration>().is_err());
    }

    #[test]
    fn rejects_too_many_segments() {
        assert!("1:2:3:4".parse::<WorkDuration>().is_err());
    }

    #[test]
    fn formats_minutes_as_hours_and_minutes() {
        assert_eq!(WorkDuration { minutes: 150 }.to_string(), "02:30");
    }

    #[test]
    fn formats_a_full_day_with_the_days_segment() {
        assert_eq!(WorkDuration { minutes: MINUTES_IN_DAY }.to_string(), "01:00:00");
    }
}
