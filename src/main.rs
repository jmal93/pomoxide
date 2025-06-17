mod parser;
mod progress_bar;

use parser::{Cli, Command, Parser};
use progress_bar::start_work_bar;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { duration } => {
            println!("Começou um trabalho de {} minutos", duration / 60);
            start_work_bar(duration);
        }
        Command::Break { duration } => {
            println!("Começou um descanso de {} minutos", duration / 60);
        }
    }
}
