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
    if let Some(pos) = time.find('s') {
        time[..pos].parse().unwrap()
    } else {
        panic!("Formato de tempo inválido");
    }
}
