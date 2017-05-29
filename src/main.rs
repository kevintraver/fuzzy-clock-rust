extern crate chrono;
use chrono::prelude::*;

fn main() {
    let mut state: u8 = 0;
    println!("Hello, world!");
    getStep();
}

fn getStep() -> u32 {
    let local: DateTime<Local> = Local::now();

    // Compute the current 30 seconds step of the current hour
    let step: u32 = local.minute() * 2 + local.second() / 30;

    // Next state of internal clock, initialized with information about current hour of day
    let mut next_state: u32 = local.hour() * 100;

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

    println!("{:?}", next_state);
    next_state

}
