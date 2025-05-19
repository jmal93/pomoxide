mod parser;
use clap::Parser;


fn main() {
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
