use std::cmp::PartialEq;

use super::round::Round;

#[derive(Debug, PartialEq)]
pub struct Game {
    id: i32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn new(id: i32, cubes: Vec<Round>) -> Self {
        Game { id, rounds: cubes }
    }

    pub fn parse(str: &str) -> Game {
        let splitted = str.split(':').map(|s| s.trim()).collect::<Vec<&str>>();
        let str = splitted[0].split_ascii_whitespace().last().unwrap();
        let id = str.parse().unwrap();
        let rounds = Round::parse_vec(splitted[1]);
        Game::new(id, rounds)
    }

    pub fn parse_vec(lines: &[&str]) -> Vec<Game> {
        lines.iter().map(|line| Game::parse(line)).collect()
    }

    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| round.is_possible())
    }

    pub fn sum_of_possble_games(games: &[Game]) -> i32 {
        games
            .iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
            .sum()
    }
}
