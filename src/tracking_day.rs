use serde::{Deserialize, Serialize};
use chrono::{Locale, NaiveDate};

use crate::TrackingRecord;

#[derive(Serialize,Deserialize)]
pub struct TrackingDay {
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    pub tracking: Vec<TrackingRecord>
}

// Warning: the dash separator, which means ignore trailing 0, is not portable and is glibc dependant. Reead glibcn notes at the bottom of man 3 sfrtime page.
const DATE_FORMAT : &str = "%A %-d %B %Y";
const DATE_LOCALE : Locale = Locale::fr_FR;

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
        serializer.collect_str(&date.format_localized(DATE_FORMAT, DATE_LOCALE).to_string())
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de> {
        let date_str : &str = Deserialize::deserialize(deserializer)?;

        let parts: Vec<&str> = date_str.split(" ").collect();

        let year : i32 = parts[3].parse().unwrap();
        let month : u32 = (month_to_u8(parts[2]) + 1).into();
        let day : u32 = parts[1].parse().unwrap();

        Ok(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}

fn month_to_u8(input: &str) -> u8 {
    match input.to_lowercase() {
        s if s.starts_with("ja") => 0,
        s if s.starts_with("f") => 1,
        s if s.starts_with("mar") => 2,
        s if s.starts_with("av") => 3,
        s if s.starts_with("mai") => 4,
        s if s.starts_with("juin") => 5,
        s if s.starts_with("juil") => 6,
        s if s.starts_with("ao") => 7,
        s if s.starts_with("s") => 8,
        s if s.starts_with("o") => 9,
        s if s.starts_with("n") => 10,
        s if s.starts_with("d") => 11,
        _ => panic!("Could not parse month: {}", input),
    }
}
