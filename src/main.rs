mod parser;
mod progress_bar;

use parser::{Cli, Command, Parser};
use progress_bar::start_bar;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { duration } => {
            println!("Começou um pomodoro de {duration}");
            start_bar(duration);
        }
        Command::Finish => {
            println!("Terminou o pomodoro atual");
        }
        Command::Clear => {
            println!("Limpou o pomodoro atual");
        }
    }
}
