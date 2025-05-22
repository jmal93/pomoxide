mod parser;
use parser::{ Cli, Command, Parser };

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
}

