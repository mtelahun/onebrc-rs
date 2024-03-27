use std::env;

use onebrc_rs::station_measurements::StationMeasurements;

fn main() {
    let mut args = env::args();
    if args.len() < 2 || args.len() > 2 {
        eprintln!("usage: calculate_average PATH");
    }
    let path = args.next_back().unwrap();
    let mut measurements =
        StationMeasurements::from_file(&path).expect("failed to open measurements file");

    measurements.read_lines();

    println!("{}", measurements.get_all_stats());
    println!("{} cities processed", measurements.len());
}
