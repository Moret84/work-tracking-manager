mod tracking_record;
mod work_duration;
mod tracking_day;

use tracking_day::TrackingDay;
use tracking_record::TrackingRecord;
use work_duration::WorkDuration;

use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
struct Cli {
    /// The output path where saving the result.
    #[arg(short, long)]
    output_path: Option<PathBuf>,

    /// The path of the input file.
    input_path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let tracking_days = load_tracking_days(args.input_path.to_str().unwrap());
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
