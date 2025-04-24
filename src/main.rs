use chrono::TimeDelta;
use clap::Parser;
use indicatif::ProgressBar;
use std::{thread, time::Duration};
use timer::Timer;

#[derive(Parser)]
struct Cli {
    time: String,
}

fn main() {
    let args = Cli::parse();
    let time = convert_time(args.time);

    let bar = ProgressBar::new(time);

    let timer = Timer::new();
    let duration = Duration::from_secs(1);
    let timedelta = TimeDelta::from_std(duration).unwrap();
    let guard1 = timer.schedule_repeating(timedelta, move || {
        bar.inc(1);
    });

    thread::sleep(Duration::from_secs(time));

    drop(guard1);
}

fn convert_time(time: String) -> u64 {
    let mut total_seconds = 0;
    let mut current_number = 0;

    for c in time.chars() {
        if c.is_ascii_digit() {
            current_number = current_number * 10 + c.to_digit(10).unwrap() as u64;
        } else if c == 'm' {
            total_seconds += 60 * current_number;
            current_number = 0;
        } else if c == 's' {
            total_seconds += current_number;
            current_number = 0;
        }
    }

    total_seconds
}
