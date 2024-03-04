mod tracking_record;
mod work_duration;
mod tracking_day;

use tracking_day::TrackingDay;
use tracking_record::TrackingRecord;
use work_duration::WorkDuration;
use std::collections::BTreeMap;

use chrono::{Datelike,NaiveDate};
use std::fs;

use clap::Parser;
use std::collections::HashMap;
use std::path::{Path,PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Parser)]
struct Cli {
    /// The output path where saving the result.
    #[arg(short, long)]
    output_path: Option<PathBuf>,

    /// The filter to use for the query
    #[arg(short, long)]
    filter_id: Option<String>,

    #[arg(short, long, action)]
    remove_empty: bool,

    #[arg(short, long, action)]
    total: bool,

    /// Whether to write back the result or not.
    #[arg(short, long, action)]
    write: bool,

    /// The path of the input file.
    input_path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut tracking_days = load_tracking_days(args.input_path.to_str().unwrap());

    if let Some(filter_id) = args.filter_id {
        filter_id_starts_with(&mut tracking_days, &filter_id);
    }

    if args.remove_empty {
        filter_remove_empty(&mut tracking_days);
    }

    if args.write {
        save(&tracking_days);
    }

    if args.total {
        let tracking = total_by_id(&tracking_days);
        println!("{}", serde_yaml::to_string(&tracking).unwrap());
        return;
    }

    println!("{}", serde_yaml::to_string(&tracking_days).unwrap());
}

fn filter_id_starts_with(tracking_days: &mut Vec<TrackingDay>, filter_str: &str) {
    tracking_days
        .iter_mut()
        .for_each(|tracking_day| {
            tracking_day.tracking
                .retain(|record| record.id.starts_with(&filter_str))
        });
}

fn filter_remove_empty(tracking_days: &mut Vec<TrackingDay>) {
    tracking_days.retain(|tracking_day| !tracking_day.tracking.is_empty());
}

fn total_by_id(tracking_days: &Vec<TrackingDay>) -> Vec<TrackingRecord> {
    let mut tracking = HashMap::new();

    for tracking_day in tracking_days {
        for record in &tracking_day.tracking {
            tracking.entry(record.id.to_owned())
                .and_modify(|t: &mut TrackingRecord| t.duration += record.duration)
                .or_insert(record.clone());
        }
    }

    tracking
        .values()
        .cloned()
        .collect()
}

fn load_tracking_days(filepath: &str) -> Vec<TrackingDay> {
    let mut file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("Erreur lors de l'ouverture du fichier : {:?}", error),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            serde_yaml::from_str(&contents).unwrap()
        }
        Err(error) => panic!("Erreur lors de la lecture du fichier : {:?}", error),
    }
}

fn save(tracking_days: &Vec<TrackingDay>) {
    for tracking_day in tracking_days {
        let year = tracking_day.date.year().to_string();

        if !std::path::Path::new(&year).exists() {
            fs::create_dir(&year).unwrap();
        }

        let month = tracking_day.date.month0().to_string();

        let mut filepath = PathBuf::new();
        filepath.push(&year);
        filepath.push(&month);
        filepath.push(".yml");

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath)
            .unwrap();

        file.write_all(
            serde_yaml::to_string(&tracking_day)
                .unwrap()
                .as_bytes())
            .unwrap()
    }
}

