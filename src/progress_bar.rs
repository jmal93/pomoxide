use indicatif::ProgressBar;
use std::{thread::sleep, time::Duration};

pub fn start_bar(duration: u64) {
    let bar = ProgressBar::new(duration);

    for _ in 0..duration {
        bar.inc(1);
        sleep(Duration::from_secs(1));
    }

    bar.finish();
}
