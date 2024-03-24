use crate::day02::{color::Color, draw::Draw, game::Game, round::Round};

#[test]
fn test_example_input() {
    let input = include_str!("test.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let games = Game::from_str(&lines);
    assert_eq!(8, Game::sum_of_possble_games(&games))
}

#[test]
fn test_input() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let games = Game::from_str(&lines);
    assert_eq!(2285, Game::sum_of_possble_games(&games))
}

#[test]
fn test_parse_draw() {
    assert_eq!(Draw::new(1, Color::Red), Draw::parse("1 red"));
}

#[test]
fn test_parse_draw_vec() {
    let s = "1 red, 2 blue, 3 green";
    let expected = vec![
        Draw::new(1, Color::Red),
        Draw::new(2, Color::Blue),
        Draw::new(3, Color::Green),
    ];
    assert_eq!(expected, Draw::parse_vec(s));
}

#[test]
fn test_parse_game() {
    let s = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let expected = Game::new(
        5,
        vec![
            Round::new(vec![
                Draw::new(6, Color::Red),
                Draw::new(1, Color::Blue),
                Draw::new(3, Color::Green),
            ]),
            Round::new(vec![
                Draw::new(2, Color::Blue),
                Draw::new(1, Color::Red),
                Draw::new(2, Color::Green),
            ]),
        ],
    );
    assert_eq!(expected, Game::parse(s));
}

#[test]
fn test_minimum_needed_cubes() {
    let input = include_str!("test.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let games = Game::from_str(&lines);
    let sum_of_minimun_needed_cubes = Game::sum_of_minimun_needed_cubes(&games);
    assert_eq!(2286, sum_of_minimun_needed_cubes)
}

#[test]
fn test_minimum_needed_cubes_real() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let games = Game::from_str(&lines);
    let sum_of_minimun_needed_cubes = Game::sum_of_minimun_needed_cubes(&games);
    assert_eq!(77021, sum_of_minimun_needed_cubes)
}
