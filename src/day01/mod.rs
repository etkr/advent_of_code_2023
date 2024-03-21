use std::cmp::{max, min};

fn find_numeric_chars(str: &str) -> Vec<i32> {
    str.chars()
        .filter_map(|s| s.to_digit(10))
        .map(|u| u as i32)
        .collect()
}

fn add_first_last_digit(vec: &[i32]) -> i32 {
    let first = vec.first().expect("No first element");
    let last = vec.last().expect("No last element");
    let result: String = format!("{}{}", first, last);
    result.parse().expect("Cant parse int")
}

pub fn find_calibration_value(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|s| find_numeric_chars(s))
        .map(|s| add_first_last_digit(&s))
        .sum()
}

fn check_digit_spell(str: &str) -> Option<i32> {
    match str {
        _ if str.starts_with("one") => Some(1),
        _ if str.starts_with("two") => Some(2),
        _ if str.starts_with("three") => Some(3),
        _ if str.starts_with("four") => Some(4),
        _ if str.starts_with("five") => Some(5),
        _ if str.starts_with("six") => Some(6),
        _ if str.starts_with("seven") => Some(7),
        _ if str.starts_with("eight") => Some(8),
        _ if str.starts_with("nine") => Some(9),
        _ => None,
    }
}

pub fn find_digit_with_str(str: &str) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();

    for start in 0..str.len() {
        let char = str.as_bytes()[start] as char;
        let digit = char.to_digit(10);

        if let Some(int) = digit {
            results.push(int as i32);
        }

        /* clamp end of window */
        let end = min(start + 5, str.len());
        let window = &str[start..end];
        let str_digit = check_digit_spell(window);

        if let Some(int) = str_digit {
            results.push(int)
        }
    }

    results
}

pub fn find_calibration_value_2(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|s| find_digit_with_str(s))
        .map(|s| add_first_last_digit(&s))
        .sum()
}

#[cfg(test)]
mod test;
