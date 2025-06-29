use std::u64;

pub use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
#[command(name = "Pomoxide")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    Start {
        #[arg(long, default_value_t = 25 * 60, value_parser = convert_time)]
        duration: u64,
    },
    Break {
        #[arg(long, default_value_t = 5 * 60, value_parser = convert_time)]
        duration: u64,
    },
}

pub fn convert_time(s: &str) -> Result<u64, String> {
    if let Ok(seconds) = s.parse::<u64>() {
        return Ok(seconds);
    }

    let mut total_seconds: u64 = 0;
    let mut current_value = String::new();
    let mut has_minutes: bool = false;
    let mut has_seconds: bool = false;
    let mut last_digit: char = ' ';

    for c in s.chars() {
        match c {
            '0'..'9' => current_value.push(c),
            'm' => {
                if current_value.is_empty() {
                    return Err("Value missing for minutes".to_string());
                }
                total_seconds += current_value.parse::<u64>().unwrap() * 60;
                if last_digit == 's' {
                    return Err("Wrong format, minutes must come before seconds".to_string());
                }
                last_digit = c;
                has_minutes = true;
                current_value.clear();
            }
            's' => {
                if current_value.is_empty() {
                    return Err("Value missing for seconds".to_string());
                }
                let seconds = current_value.parse::<u64>().unwrap();
                if seconds >= 60 {
                    return Err("Max value for seconds must be 59".to_string());
                }
                total_seconds += seconds;
                last_digit = c;
                has_seconds = true;
                current_value.clear();
            }
            _ => return Err(format!("Character not valid: {}", c)),
        };
    }

    if !current_value.is_empty() && !has_seconds && !has_minutes {
        return Err("Timestamps not specified".to_string());
    }
    Ok(total_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parser_start() {
        let args = ["pomoxide", "start"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.command, Command::Start { duration: 25 * 60 });
    }

    #[test]
    fn test_cli_parser_start_duration() {
        let args = ["pomoxide", "start", "--duration", "10m"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.command, Command::Start { duration: 10 * 60 });
    }

    #[test]
    fn test_cli_parser_break() {
        let args = ["pomoxide", "break"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.command, Command::Break { duration: 5 * 60 });
    }

    #[test]
    fn test_convert_time_accept_number() {
        assert_eq!(convert_time("1500"), Ok(1500));
    }

    #[test]
    fn test_convert_time_10m() {
        assert_eq!(convert_time("10m"), Ok(600));
    }

    #[test]
    fn test_convert_time_10s() {
        assert_eq!(convert_time("10s"), Ok(10));
    }

    #[test]
    fn test_convert_time_10m10s() {
        assert_eq!(convert_time("10m10s"), Ok(610));
    }

    #[test]
    fn test_convert_time_a() {
        assert_eq!(convert_time("a"), Err("Character not valid: a".to_string()));
    }

    #[test]
    fn test_convert_time_10s10m() {
        assert_eq!(
            convert_time("10s10m"),
            Err("Wrong format, minutes must come before seconds".to_string())
        );
    }

    #[test]
    fn test_convert_time_61s() {
        assert_eq!(
            convert_time("61s"),
            Err("Max value for seconds must be 59".to_string())
        );
    }

    #[test]
    fn test_convert_time_10m61s() {
        assert_eq!(
            convert_time("10m61s"),
            Err("Max value for seconds must be 59".to_string())
        );
    }

    #[test]
    fn test_convert_time_m() {
        assert_eq!(
            convert_time("m"),
            Err("Value missing for minutes".to_string())
        );
    }

    #[test]
    fn test_convert_time_m10s() {
        assert_eq!(
            convert_time("m10s"),
            Err("Value missing for minutes".to_string())
        );
    }

    #[test]
    fn test_convert_time_s() {
        assert_eq!(
            convert_time("s"),
            Err("Value missing for seconds".to_string())
        );
    }

    #[test]
    fn test_convert_time_10ms() {
        assert_eq!(
            convert_time("10ms"),
            Err("Value missing for seconds".to_string())
        );
    }
}
