use std::u64;

pub fn convert_time(s: &str) -> Result<u64, String> {
    let mut total_seconds = 0;
    let mut current_value = String::new();
    for c in s.chars() {
        match c {
            '0'..'9' => current_value.push(c),
            'm' => {
                total_seconds += current_value.parse::<u64>().unwrap() * 60;
                current_value.clear();
            }
            's' => {
                total_seconds += current_value.parse::<u64>().unwrap();
                current_value.clear();
            }
            _ => return Err("erro".to_string()),
        };
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
}
