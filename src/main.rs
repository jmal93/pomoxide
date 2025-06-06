mod opf;
mod parser;

use parser::{Cli, Command, Parser};
use time::{OffsetDateTime, format_description};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { duration } => {
            println!("Começou um pomodoro de {duration}");
        }
        Command::Finish => {
            println!("Terminou o pomodoro atual");
        }
        Command::Clear => {
            println!("Limpou o pomodoro atual");
        }
    }

    let datetime = OffsetDateTime::now_local().unwrap();

    let format =
        format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]").unwrap();

    let formatted = datetime.format(&format).unwrap();

    println!("{}", formatted);
}
