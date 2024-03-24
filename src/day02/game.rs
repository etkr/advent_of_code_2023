use core::str;
use std::{cmp::PartialEq, collections::HashMap};

use super::{color::Color, round::Round};

pub struct Bag {
    red: i32,
    blue: i32,
    green: i32,
}

impl Bag {
    pub fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

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

    pub fn from_str(lines: &[&str]) -> Vec<Game> {
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

    fn minimun_needed_cubes(&self) -> Bag {
        let mut dict = HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);

        for round in &self.rounds {
            for draw in &round.draw {
                let color = draw.color;
                let quantity = draw.quantity;
                if dict[&color] < quantity {
                    dict.insert(color, quantity);
                }
            }
        }

        Bag {
            red: dict[&Color::Red],
            blue: dict[&Color::Blue],
            green: dict[&Color::Green],
        }
    }

    pub fn sum_of_minimun_needed_cubes(games: &[Game]) -> i32 {
        games
            .iter()
            .map(|game| game.minimun_needed_cubes().power())
            .sum()
    }
}
