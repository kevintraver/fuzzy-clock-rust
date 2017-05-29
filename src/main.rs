extern crate chrono;
use chrono::prelude::*;
use std::collections::BTreeMap;

fn main() {
    let now: DateTime<Local> = Local::now();
    let state = get_state(now);
    let title = current_title(state, now);
    println!("{}",title);
}

fn current_title(state: u32, now: DateTime<Local>) -> String {
    let fuzzy_map = generate_fuzzy_map();

    // Time descriptions may refer to the current or the following hour
    let mut hour_offset: u32 = 0;
    if fuzzy_map.get(format!("S{:02}h", state % 100).as_str()).unwrap() == "next" {
        hour_offset = 1;
    }

    // Build the fuzzy time description
   let format = fuzzy_map.get(format!("S{:02}", state % 100).as_str()).unwrap();
   let hour_name = fuzzy_map.get(format!("H{:02}", (now.hour() + hour_offset) % 12).as_str()).unwrap();

   format.replace("{}",hour_name)
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

fn generate_fuzzy_map() -> BTreeMap<String, String> {
    let mut fuzzy_map = BTreeMap::new();
    fuzzy_map.insert("S00".to_string(), "{} o’clock".to_string());
    fuzzy_map.insert("S01".to_string(), "shortly after {}".to_string());
    fuzzy_map.insert("S02".to_string(), "five past {}".to_string());
    fuzzy_map.insert("S03".to_string(), "ten past {}".to_string());
    fuzzy_map.insert("S04".to_string(), "quarter past {}".to_string());
    fuzzy_map.insert("S05".to_string(), "twenty past {}".to_string());
    fuzzy_map.insert("S06".to_string(), "twentyfive past {}".to_string());
    fuzzy_map.insert("S07".to_string(), "half past {}".to_string());
    fuzzy_map.insert("S08".to_string(), "twentyfive to {}".to_string());
    fuzzy_map.insert("S09".to_string(), "twenty to {}".to_string());
    fuzzy_map.insert("S10".to_string(), "quarter to {}".to_string());
    fuzzy_map.insert("S11".to_string(), "ten to {}".to_string());
    fuzzy_map.insert("S12".to_string(), "five to {}".to_string());
    fuzzy_map.insert("S13".to_string(), "nearly {}".to_string());
    fuzzy_map.insert("S00h".to_string(), "current".to_string());
    fuzzy_map.insert("S01h".to_string(), "current".to_string());
    fuzzy_map.insert("S02h".to_string(), "current".to_string());
    fuzzy_map.insert("S03h".to_string(), "current".to_string());
    fuzzy_map.insert("S04h".to_string(), "current".to_string());
    fuzzy_map.insert("S05h".to_string(), "current".to_string());
    fuzzy_map.insert("S06h".to_string(), "current".to_string());
    fuzzy_map.insert("S07h".to_string(), "current".to_string());
    fuzzy_map.insert("S08h".to_string(), "next".to_string());
    fuzzy_map.insert("S09h".to_string(), "next".to_string());
    fuzzy_map.insert("S10h".to_string(), "next".to_string());
    fuzzy_map.insert("S11h".to_string(), "next".to_string());
    fuzzy_map.insert("S12h".to_string(), "next".to_string());
    fuzzy_map.insert("S13h".to_string(), "next".to_string());
    fuzzy_map.insert("H00".to_string(), "twelve".to_string());
    fuzzy_map.insert("H01".to_string(), "one".to_string());
    fuzzy_map.insert("H02".to_string(), "two".to_string());
    fuzzy_map.insert("H03".to_string(), "three".to_string());
    fuzzy_map.insert("H04".to_string(), "four".to_string());
    fuzzy_map.insert("H05".to_string(), "five".to_string());
    fuzzy_map.insert("H06".to_string(), "six".to_string());
    fuzzy_map.insert("H07".to_string(), "seven".to_string());
    fuzzy_map.insert("H08".to_string(), "eight".to_string());
    fuzzy_map.insert("H09".to_string(), "nine".to_string());
    fuzzy_map.insert("H10".to_string(), "ten".to_string());
    fuzzy_map.insert("H11".to_string(), "eleven".to_string());
    fuzzy_map.insert("M00".to_string(), "midnight".to_string());
    fuzzy_map.insert("M12".to_string(), "noon".to_string());
    fuzzy_map.insert("D00".to_string(), "sunday".to_string());
    fuzzy_map.insert("D01".to_string(), "monday".to_string());
    fuzzy_map.insert("D02".to_string(), "tuesday".to_string());
    fuzzy_map.insert("D03".to_string(), "wednesday".to_string());
    fuzzy_map.insert("D04".to_string(), "thursday".to_string());
    fuzzy_map.insert("D05".to_string(), "friday".to_string());
    fuzzy_map.insert("D06".to_string(), "saturday".to_string());
    fuzzy_map
}
