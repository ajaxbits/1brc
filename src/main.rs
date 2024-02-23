use model::StationEntry;
use rayon::prelude::*;
use rayon::str::ParallelString;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::{fs, mem};

mod model;

fn main() {
    println!("Reading file to string");

    let data = fs::read_to_string("data/measurements.txt").unwrap();
    let rows = data.par_lines();

    let stations: Arc<Mutex<BTreeMap<String, StationEntry>>> =
        Arc::new(Mutex::new(BTreeMap::new()));
    rows.for_each(|row| {
        let (station, temp) = row.split_once(';').unwrap();
        let station = station.to_string();
        let temp: i64 = temp.replace('.', "").parse().unwrap();

        stations
            .lock()
            .unwrap()
            .entry(station)
            .and_modify(|entry| {
                entry.running_sum += 1;
                entry.count += 1;
                entry.entries.insert(temp);
            })
            .or_insert(StationEntry::new(temp, 1, BTreeSet::from([temp])));
    });

    let mut stations: BTreeMap<String, StationEntry> = mem::take(&mut stations.lock().unwrap());

    while let Some(item) = stations.pop_first() {
        let name = item.0;
        let min = item.1.entries.first().unwrap();
        let max = item.1.entries.last().unwrap();
        let mean = item.1.running_sum / item.1.count;

        let min = convert_to_dec_string(*min);
        let max = convert_to_dec_string(*max);
        let mean = convert_to_dec_string(mean);

        println!("{name}={min}/{mean}/{max}");
    }

    println!("Read File!");
}

fn convert_to_dec_string(temp: i64) -> String {
    let mut temp = format!("{temp}");
    let pos = temp.len().saturating_sub(1);

    temp.insert(pos, '.');

    temp
}
