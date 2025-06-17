use indicatif::{ProgressBar, ProgressStyle};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub fn start_work_bar(duration: u64) {
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template("{bar}{elapsed}")
            .unwrap()
            .with_key(
                "elapsed",
                |state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                    let elapsed_secs = state.elapsed().as_secs();
                    write!(w, "{:02}:{:02}", elapsed_secs / 60, elapsed_secs % 60).unwrap()
                },
            ),
    );

    let start = Instant::now();
    let interval = Duration::from_millis(100);

    loop {
        let elapsed = start.elapsed().as_secs_f64();
        let progress = (elapsed / duration as f64 * 100.0).min(100.0) as u64;

        bar.set_position(progress);

        if progress >= 100 {
            break;
        }

        sleep(interval);
    }
}
