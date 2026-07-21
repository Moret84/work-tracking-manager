use serde::{Deserialize, Serialize};
use chrono::{Locale, NaiveDate};

use crate::TrackingRecord;

#[derive(Serialize,Deserialize)]
pub struct TrackingDay {
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    pub tracking: Vec<TrackingRecord>
}

// Warning: the dash separator, which means ignore trailing 0, is not portable and is glibc dependent as said in glibc notes at the bottom of man 3 sfrtime page.
// We then implement a custom month parser for deserialization.
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
        let date_str = String::deserialize(deserializer)?;
        parse_french_date(&date_str).map_err(serde::de::Error::custom)
}

fn parse_french_date(date_str: &str) -> Result<NaiveDate, String> {
    let parts: Vec<&str> = date_str.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(format!(
            "date invalide « {date_str} » : format attendu « jour_semaine jour mois année » (ex. « lundi 13 juillet 2026 »)"
        ));
    }

    let day: u32 = parts[1].parse()
        .map_err(|_| format!("jour invalide « {} » dans la date « {date_str} »", parts[1]))?;
    let month = u32::from(month_index(parts[2])? + 1);
    let year: i32 = parts[3].parse()
        .map_err(|_| format!("année invalide « {} » dans la date « {date_str} »", parts[3]))?;

    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| format!("date inexistante : « {date_str} »"))
}

impl TrackingDay {
    pub fn add_tracking_record(&mut self, record_to_add: TrackingRecord) {
        let existing_record = self.tracking.iter_mut()
            .find(|record| record.id == record_to_add.id);

        match existing_record {
            Some(value) => value.duration += record_to_add.duration,
            None => self.tracking.push(record_to_add)
        }
    }
}

fn month_index(input: &str) -> Result<u8, String> {
    match input.to_lowercase() {
        s if s.starts_with("ja") => Ok(0),
        s if s.starts_with("f") => Ok(1),
        s if s.starts_with("mar") => Ok(2),
        s if s.starts_with("av") => Ok(3),
        s if s.starts_with("mai") => Ok(4),
        s if s.starts_with("juin") => Ok(5),
        s if s.starts_with("juil") => Ok(6),
        s if s.starts_with("ao") => Ok(7),
        s if s.starts_with("s") => Ok(8),
        s if s.starts_with("o") => Ok(9),
        s if s.starts_with("n") => Ok(10),
        s if s.starts_with("d") => Ok(11),
        _ => Err(format!("mois inconnu : « {input} »")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_french_date() {
        let date = parse_french_date("jeudi 09 juillet 2026").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 7, 9).unwrap());
    }

    #[test]
    fn ignores_the_weekday_name() {
        let date = parse_french_date("nimportequoi 1 janvier 2026").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
    }

    #[test]
    fn rejects_a_date_with_a_missing_field() {
        assert!(parse_french_date("lundi 13 juillet").is_err());
    }

    #[test]
    fn rejects_an_unknown_month() {
        assert!(parse_french_date("lundi 13 xyz 2026").is_err());
    }

    #[test]
    fn rejects_a_non_numeric_day() {
        assert!(parse_french_date("lundi treize juillet 2026").is_err());
    }

    #[test]
    fn rejects_a_nonexistent_calendar_date() {
        assert!(parse_french_date("lundi 30 fevrier 2026").is_err());
    }
}
