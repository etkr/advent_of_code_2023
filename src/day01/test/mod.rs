use crate::day01::{
    add_first_last_digit, find_calibration_value, find_calibration_value_2, find_digit_with_str,
};

#[test]
fn find_digit_names_should_be_none() {
    assert_eq!(vec![2, 1, 9], find_digit_with_str("two1nine"))
}

#[test]
fn test_add_first_last_digit() {
    assert_eq!(12, add_first_last_digit(&[1, 2]))
}

#[test]
fn test_example_input() {
    let input = include_str!("test.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let value = find_calibration_value(&lines);

    assert_eq!(142, value)
}

#[test]
fn test_input_part1() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let value = find_calibration_value(&lines);

    assert_eq!(54159, value)
}

#[test]
fn test_input_part2() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let value = find_calibration_value_2(&lines);

    assert_eq!(53866, value)
}
