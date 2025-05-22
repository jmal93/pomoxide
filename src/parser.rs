use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
#[command(name = "Pomoxide")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    Start {
        #[arg(long, default_value_t = String::from("25m"))]
        duration: String,
    },
    Finish,
    Clear,
}

pub fn convert_time(s: &str) -> Result<u64, &str> {
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
                    return Err("Value missing for minutes");
                }
                total_seconds += current_value.parse::<u64>().unwrap() * 60;
                if last_digit == 's' {
                    return Err("Wrong format, minutes must come before seconds");
                }
                last_digit = c;
                has_minutes = true;
                current_value.clear();
            }
            's' => {
                if current_value.is_empty() {
                    return Err("Value missing for seconds");
                }
                let seconds = current_value.parse::<u64>().unwrap();
                if seconds >= 60 {
                    return Err("Max value for seconds must be 59");
                }
                total_seconds += seconds;
                last_digit = c;
                has_seconds = true;
                current_value.clear();
            }
            _ => return Err("Character not valid"),
        };
    }

    if !current_value.is_empty() && !has_seconds && !has_minutes {
        return Err("Timestamps not specified");
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

        assert_eq!(
            cli.command,
            Command::Start {
                duration: String::from("25m")
            }
        );
    }

    #[test]
    fn test_cli_parser_start_duration() {
        let args = ["pomoxide", "start", "--duration", "10m"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(
            cli.command,
            Command::Start {
                duration: String::from("10m")
            }
        );
    }

    #[test]
    fn test_cli_parser_finish() {
        let args = ["pomoxide", "finish"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.command, Command::Finish);
    }

    #[test]
    fn test_cli_parser_clear() {
        let args = ["pomoxide", "clear"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.command, Command::Clear);
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
        assert_eq!(convert_time("a"), Err("Character not valid"));
    }

    #[test]
    fn test_convert_time_10() {
        assert_eq!(convert_time("10"), Err("Timestamps not specified"));
    }

    #[test]
    fn test_convert_time_10s10m() {
        assert_eq!(
            convert_time("10s10m"),
            Err("Wrong format, minutes must come before seconds")
        );
    }

    #[test]
    fn test_convert_time_61s() {
        assert_eq!(convert_time("61s"), Err("Max value for seconds must be 59"));
    }

    #[test]
    fn test_convert_time_10m61s() {
        assert_eq!(
            convert_time("10m61s"),
            Err("Max value for seconds must be 59")
        );
    }

    #[test]
    fn test_convert_time_m() {
        assert_eq!(convert_time("m"), Err("Value missing for minutes"));
    }

    #[test]
    fn test_convert_time_m10s() {
        assert_eq!(convert_time("m10s"), Err("Value missing for minutes"));
    }

    #[test]
    fn test_convert_time_s() {
        assert_eq!(convert_time("s"), Err("Value missing for seconds"));
    }

    #[test]
    fn test_convert_time_10ms() {
        assert_eq!(convert_time("10ms"), Err("Value missing for seconds"));
    }
}
