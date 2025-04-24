use chrono::TimeDelta;
use clap::Parser;
use indicatif::ProgressBar;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use timer::Timer;

#[derive(Parser)]
struct Cli {
    time: String,
}

fn main() {
    let args = Cli::parse();
    let time = match convert_time(args.time) {
        Ok(t) => t,
        Err(error) => {
            eprint!("Erro ao converter tempo:\n{}", error);
            std::process::exit(1);
        }
    };

    let bar = Arc::new(Mutex::new(ProgressBar::new(time)));

    let timer = Timer::new();
    let duration = Duration::from_secs(1);
    let timedelta = TimeDelta::from_std(duration).unwrap();

    let bar_clone = Arc::clone(&bar);
    let guard1 = timer.schedule_repeating(timedelta, move || {
        bar_clone.lock().unwrap().inc(1);
    });

    thread::sleep(Duration::from_secs(time));

    bar.lock()
        .unwrap()
        .finish_with_message("Pomodoro concluído!");

    drop(guard1);
}

fn convert_time(time: String) -> Result<u64, String> {
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
        } else {
            return Err(format!("Caractere inválido: '{}'\n", c));
        }
    }

    Ok(total_seconds + current_number)
}
