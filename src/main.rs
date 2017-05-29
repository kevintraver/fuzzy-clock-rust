extern crate chrono;
extern crate serde_derive;
extern crate serde_yaml;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;
use std::collections::BTreeMap;

fn main() {
    let now: DateTime<Local> = Local::now();
    let state = get_state(now);
    let title = current_title(state, now);
    println!("{}", title);
}

fn current_title(state: u32, now: DateTime<Local>) -> String {
    let fuzzy_map = read_fuzzy_map().unwrap();

    // Time descriptions may refer to the current or the following hour
    let mut hour_offset: u32 = 0;
    if fuzzy_map
           .get(format!("S{:02}h", state % 100).as_str())
           .unwrap() == "next" {
        hour_offset = 1;
    }

    // Build the fuzzy time description
    let format = fuzzy_map
        .get(format!("S{:02}", state % 100).as_str())
        .unwrap();
    let hour_name = fuzzy_map
        .get(format!("H{:02}", (now.hour() + hour_offset) % 12).as_str())
        .unwrap();

    format.replace("{}", hour_name)
}

fn get_state(now: DateTime<Local>) -> u32 {

    // Compute the current 30 seconds step of the current hour
    let step: u32 = now.minute() * 2 + now.second() / 30;

    // Next state of internal clock, initialized with information about current hour of day
    let next_state: u32;

    // ...during the first minute we stick at the full hour
    if step < 2 {

        // ...during the first minute we stick at the full hour
        next_state = 0;
    } else if 2 <= step && step <= 5 {

        // ...special state before the first full 5 minutes
        next_state = 1;
    } else if step < 116 {

        // ...rounding to full 5 minute steps
        next_state = 1 + ((step + 4) / 10);
    } else {

        // ...round to full next hour
        next_state = 13;
    }

    // println!("{:?}", next_state);
    next_state

}

fn read_fuzzy_map() -> io::Result<BTreeMap<String, String>> {
    let mut file = File::open("fuzzy_map.yml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let fuzzy_map: BTreeMap<String, String> = serde_yaml::from_str(&contents).unwrap();
    Ok(fuzzy_map)
}
