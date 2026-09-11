//! Neutral JSON view of the tracking days, meant to be piped into another tool.
//!
//! The YAML representation is tailored for hand editing: French dates and `HH:MM`
//! durations. Both are awkward to re-parse, so consumers get ISO dates and integer
//! minutes instead. `minutesPerDay` travels with the payload so a consumer never has
//! to hardcode the length of a work day.

use serde::Serialize;

use crate::tracking_day::TrackingDay;
use crate::work_duration::MINUTES_IN_DAY;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Serialize)]
pub struct Export {
    #[serde(rename = "minutesPerDay")]
    minutes_per_day: u32,
    days: Vec<ExportDay>,
}

#[derive(Serialize)]
struct ExportDay {
    date: String,
    tracking: Vec<ExportRecord>,
}

#[derive(Serialize)]
struct ExportRecord {
    id: String,
    minutes: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
}

impl Export {
    pub fn new(tracking_days: &[TrackingDay]) -> Export {
        Export {
            minutes_per_day: MINUTES_IN_DAY,
            days: tracking_days.iter().map(ExportDay::new).collect(),
        }
    }
}

impl ExportDay {
    fn new(tracking_day: &TrackingDay) -> ExportDay {
        ExportDay {
            date: tracking_day.date.format(DATE_FORMAT).to_string(),
            tracking: tracking_day
                .tracking
                .iter()
                .map(|record| ExportRecord {
                    id: record.id.to_owned(),
                    minutes: record.duration.minutes,
                    description: record.description.to_owned(),
                })
                .collect(),
        }
    }
}

pub fn serialize_json(export: &Export) -> Result<String, String> {
    serde_json::to_string_pretty(export)
        .map_err(|error| format!("erreur de sérialisation JSON : {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracking_record::TrackingRecord;
    use crate::work_duration::WorkDuration;
    use chrono::NaiveDate;

    fn day(year: i32, month: u32, day: u32, records: Vec<TrackingRecord>) -> TrackingDay {
        TrackingDay {
            date: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
            tracking: records,
        }
    }

    fn record(id: &str, minutes: u32) -> TrackingRecord {
        TrackingRecord {
            id: id.to_owned(),
            description: String::new(),
            duration: WorkDuration { minutes },
        }
    }

    #[test]
    fn exports_dates_in_iso_format() {
        let export = Export::new(&[day(2026, 8, 25, vec![record("trinoma", 180)])]);
        assert_eq!(export.days[0].date, "2026-08-25");
    }

    #[test]
    fn exports_durations_as_whole_minutes() {
        let export = Export::new(&[day(2026, 8, 25, vec![record("methodo", 150)])]);
        assert_eq!(export.days[0].tracking[0].minutes, 150);
    }

    #[test]
    fn declares_the_length_of_a_work_day() {
        let export = Export::new(&[]);
        assert_eq!(export.minutes_per_day, MINUTES_IN_DAY);
    }

    #[test]
    fn keeps_every_record_of_a_day_separate() {
        let export = Export::new(&[day(
            2026,
            8,
            25,
            vec![
                record("trinoma", 180),
                record("methodo", 150),
                record("interne", 120),
            ],
        )]);
        let ids: Vec<&str> = export.days[0]
            .tracking
            .iter()
            .map(|record| record.id.as_str())
            .collect();
        assert_eq!(ids, vec!["trinoma", "methodo", "interne"]);
    }

    #[test]
    fn omits_an_empty_description() {
        let export = Export::new(&[day(2026, 8, 25, vec![record("trinoma", 180)])]);
        let json = serialize_json(&export).unwrap();
        assert!(!json.contains("description"), "{json}");
    }

    #[test]
    fn keeps_a_filled_description() {
        let mut with_description = record("trinoma", 180);
        with_description.description = "revue de code".to_owned();
        let export = Export::new(&[day(2026, 8, 25, vec![with_description])]);
        let json = serialize_json(&export).unwrap();
        assert!(json.contains("revue de code"), "{json}");
    }
}
