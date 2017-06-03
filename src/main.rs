#![feature(test)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate test;
extern crate chrono;
extern crate chrono_tz;
extern crate serde_yaml;
#[macro_use]
extern crate lazy_static;
extern crate clap;

use chrono::prelude::*;
use std::collections::BTreeMap;
use clap::{Arg, App};
use chrono::TimeZone;
use chrono_tz::Tz;

struct AppState {
    time_zone: String,
    execution_type: ExecutionType,
}

enum ExecutionType {
    SERVER,
    COMMAND_LINE,
}

static FUZZY_MAP_STRING: &'static str = include_str!("fuzzy_map.yml");

lazy_static! {
    static ref FUZZY_MAP: BTreeMap<String, String> =
        serde_yaml::from_str(FUZZY_MAP_STRING).unwrap();
}

#[get("/time")]
fn fuzzy() -> String {
    get_time()
}

fn main() {
    let matches = App::new("Fuzzy Clock")
        .version("0.1.0")
        .author("Kevin Traver <kevin.traver@gmail.com>")
        .about("Fuzzes your time")
        .arg(Arg::with_name("server")
            .short("s")
            .long("server")
            .help("Runs a server"))
        .arg(Arg::with_name("timezone")
            .short("t")
            .long("timezone")
            .value_name("TIMEZONE")
            .help("Sets default timezone")
            .takes_value(true))
        .get_matches();

    if matches.is_present("server") {
        rocket::ignite().mount("/fuzzy", routes![fuzzy]).launch();
    } else {
        println!("{}", get_time());
    }
}

fn get_time() -> String {
    let now: DateTime<Local> = Local::now();
    let state = get_state(now);
    current_title(state, now)
}

fn current_title(state: u32, now: DateTime<Local>) -> String {

    // Time descriptions may refer to the current or the following hour
    let hour_offset = if FUZZY_MAP
           .get(format!("S{:02}h", state % 100).as_str())
           .unwrap() == "next" {
        1
    } else {
        0
    };

    // Build the fuzzy time description
    let format = FUZZY_MAP
        .get(format!("S{:02}", state % 100).as_str())
        .unwrap();
    let hour_name = FUZZY_MAP
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


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_display_fuzzy_time(b: &mut Bencher) {
        b.iter(|| fuzzy());
    }
}
