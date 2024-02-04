mod tracking_record;
mod work_duration;

use tracking_record::TrackingRecord;
use work_duration::WorkDuration;

use std::time::Duration;

fn main() {
    let record = TrackingRecord {
        id: "legrand 11512".to_string(),
        description: "iOS version - Tool page is blank".to_string(),
        time: Duration::new(70, 0)
    };

    let serialized_yaml = serde_yaml::to_string(&record).unwrap();

    let work_duration : Result<WorkDuration, _>= "7:30".parse();

    println!("{}", work_duration.as_ref().unwrap());
    println!("{}", work_duration.unwrap().to_string());
}
