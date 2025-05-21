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
                    return Err("Value missing for minutes")
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
                    return Err("Value missing for seconds")
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
    fn convert_time_sucess() {
        let test_cases = vec![("10m", 600), ("10s", 10), ("10m10s", 610)];
        for (input, expected) in test_cases {
            assert_eq!(convert_time(input), Ok(expected));
        }
    }

    #[test]
    fn convert_time_fail() {
        let test_cases = vec![
            ("a", "Character not valid"),
            ("10", "Timestamps not specified"),
            ("10s10m", "Wrong format, minutes must come before seconds"),
            ("61s", "Max value for seconds must be 59"),
            ("10m61s", "Max value for seconds must be 59"),
            ("m", "Value missing for minutes"),
            ("m10s", "Value missing for minutes"),
            ("s", "Value missing for seconds"),
            ("10ms", "Value missing for seconds"),
        ];
        for (input, expected) in test_cases {
            assert_eq!(convert_time(input), Err(expected));
        }
    }
}
