mod json_export;
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
use std::fs::OpenOptions;
use std::io::Write;
use std::process::ExitCode;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    /// The output path where saving the result.
    #[arg(short, long)]
    output_path: Option<PathBuf>,

    /// The filter to use for the query.
    #[arg(short, long)]
    filter_id: Option<String>,

    /// Whether to remove days whose tracking is empty or not.
    #[arg(short, long)]
    remove_empty: bool,

    /// Whether to compute the total per item or not.
    #[arg(short, long)]
    total: bool,

    /// Whether to write back the result or not.
    #[arg(short, long)]
    write: bool,

    /// Whether we should show total next to result.
    #[arg(long)]
    show_total: bool,

    /// Whether to print a neutral JSON payload meant to be piped into another tool.
    #[arg(long, conflicts_with_all = ["total", "show_total", "write"])]
    json: bool,

    /// The path of the input file.
    input_path: PathBuf,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Erreur : {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = Cli::parse();

    let mut tracking_days = load_tracking_days(&args.input_path)?;

    if let Some(filter_id) = args.filter_id {
        filter_id_str(&mut tracking_days, &filter_id)?;
    }

    if args.remove_empty {
        filter_remove_empty(&mut tracking_days);
    }

    if args.json {
        let export = json_export::Export::new(&tracking_days);
        println!("{}", json_export::serialize_json(&export)?);
        return Ok(());
    }

    work_duration::set_include_total(args.show_total);

    if args.total {
        let tracking = total_by_id(&tracking_days);
        println!("{}", serialize_yaml(&tracking)?);
        return Ok(());
    }

    println!("{}", format_tracking_days_output(&tracking_days)?);

    if args.write {
        save(&tracking_days)?;
    }

    Ok(())
}

fn filter_id_str(tracking_days: &mut Vec<TrackingDay>, filter_str: &str) -> Result<(), String> {
    let regex_str = format!("^{}$", filter_str.replace("*", ".*"));

    let regex = Regex::new(&regex_str)
        .map_err(|error| format!("filtre invalide « {filter_str} » : {error}"))?;

    tracking_days
        .iter_mut()
        .for_each(|tracking_day| {
            tracking_day.tracking
                .retain(|record| regex.is_match(&record.id))
        });
    filter_remove_empty(tracking_days);
    Ok(())
}

fn filter_remove_empty(tracking_days: &mut Vec<TrackingDay>) {
    tracking_days.retain(|tracking_day| !tracking_day.tracking.is_empty());
}

fn format_tracking_days_output(tracking_days: &Vec<TrackingDay>) -> Result<String, String> {
    Ok(serialize_yaml(tracking_days)?
        .replace("\n- date:", "\n\n- date:"))
}

fn serialize_yaml<T: serde::Serialize>(value: &T) -> Result<String, String> {
    serde_yaml::to_string(value)
        .map_err(|error| format!("erreur de sérialisation YAML : {error}"))
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

fn load_tracking_days(filepath: &Path) -> Result<Vec<TrackingDay>, String> {
    let content = fs::read_to_string(filepath)
        .map_err(|error| format!("impossible de lire le fichier « {} » : {error}", filepath.display()))?;

    match filepath.extension().and_then(|extension| extension.to_str()) {
        Some("yml") | Some("yaml") => parse_yaml(&content, filepath),
        Some("csv") => parse_csv(&content, filepath),
        _ => Err(format!(
            "format de fichier non pris en charge : « {} » (attendu .yml, .yaml ou .csv)",
            filepath.display()
        )),
    }
}

fn save(tracking_days: &Vec<TrackingDay>) -> Result<(), String> {
    for tracking_day in tracking_days {
        let year = tracking_day.date.year().to_string();

        if !Path::new(&year).exists() {
            fs::create_dir(&year)
                .map_err(|error| format!("impossible de créer le dossier « {year} » : {error}"))?;
        }

        let month = format!("{:02}", tracking_day.date.month());

        let mut filepath = PathBuf::new();
        filepath.push(&year);
        filepath.push(&month);
        filepath.set_extension("yml");

        let content = serialize_yaml(&tracking_day)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filepath)
            .map_err(|error| format!("impossible d'ouvrir « {} » : {error}", filepath.display()))?;

        file.write_all(content.as_bytes())
            .map_err(|error| format!("impossible d'écrire dans « {} » : {error}", filepath.display()))?;
    }

    Ok(())
}

fn parse_csv(content: &str, filepath: &Path) -> Result<Vec<TrackingDay>, String> {
    let mut tracking_days : BTreeMap<NaiveDate, TrackingDay> = BTreeMap::new();

    for (line_index, line) in content.lines().enumerate() {
        let parts : Vec<&str> = line.trim().split(';').collect();

        let location = format!("{} ligne {}", filepath.display(), line_index + 1);

        if parts.len() < 5 {
            return Err(format!(
                "ligne CSV incomplète ({location}) : 5 champs attendus, {} trouvé(s)",
                parts.len()
            ));
        }

        let hours: f64 = parts[0].replace(",", ".").parse()
            .map_err(|_| format!("durée invalide « {} » ({location})", parts[0]))?;
        let minutes = (hours * 60.0).round() as u32;
        let id = parts[1].to_string();
        let description = format!("{} {}", parts[3], parts[2]);
        let date = NaiveDate::parse_from_str(parts[4], "%m/%d/%Y")
            .map_err(|_| format!("date invalide « {} » ({location})", parts[4]))?;

        tracking_days
            .entry(date)
            .or_insert_with(|| TrackingDay {
                date,
                tracking: Vec::new()
            })
            .add_tracking_record(TrackingRecord {
                id,
                description,
                duration: WorkDuration {
                    minutes
                }
            });
    }

    Ok(tracking_days
        .into_values()
        .collect())
}

fn parse_yaml(content: &str, filepath: &Path) -> Result<Vec<TrackingDay>, String> {
    serde_yaml::from_str(content)
        .map_err(|error| format!("erreur de lecture du YAML « {} » : {error}", filepath.display()))
}
