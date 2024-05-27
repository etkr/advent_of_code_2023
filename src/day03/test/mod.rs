use nalgebra::dmatrix;

use crate::day03::schematic::Schematic;

#[test]
fn test_parse_schematic() {
    let input = vec!["123", "456", "789"];
    let expected = dmatrix!['1', '2', '3'; '4', '5', '6'; '7', '8', '9'];
    let schematic = Schematic::from_str(&input);
    assert_eq!(expected, schematic.matrix);
}

#[test]
fn test_input() {
    let input = include_str!("test.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let schematic = Schematic::from_str(&lines);
    assert_eq!(4361, schematic.sum_of_part_numbers());
}

#[test]
fn test_real_input() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let schematic = Schematic::from_str(&lines);
    assert_eq!(529673, schematic.sum_of_part_numbers());
}

#[test]
fn test_find_part_numbers() {
    let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];

    let input = include_str!("test.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let schematic = Schematic::from_str(&lines);
    let part_numbers = schematic.find_part_numbers();

    assert_eq!(expected, part_numbers);
}

#[test]
fn test_has_adjecent_symbol() {
    let input = vec![".*.", ".21", "..."];
    let schematic = Schematic::from_str(&input);
    assert!(schematic.has_adjecent_symbol(1, 1));   
}

#[test]
fn test_has_diagonal_symbol() {
    let input = vec!["*..", ".21", "..."];
    let schematic = Schematic::from_str(&input);
    assert!(schematic.has_adjecent_symbol(1, 1));   
}

#[test]
fn test_has_adjecent_symbol_no() {
    let input = vec!["...", ".21", "..."];
    let schematic = Schematic::from_str(&input);
    assert!(!schematic.has_adjecent_symbol(1, 1));   
}


