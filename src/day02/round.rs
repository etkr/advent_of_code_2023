use super::draw::Draw;

#[derive(Debug, PartialEq)]
pub struct Round {
     draw: Vec<Draw>,
}

impl Round {
    pub fn parse_vec(s: &str) -> Vec<Round> {
        s.split(';').map(Round::parse).collect()
    }

    fn parse(s: &str) -> Self {
        let draw = Draw::parse_vec(s);
        Round::new(draw)
    }

    pub fn new(draw: Vec<Draw>) -> Self {
        Round { draw }
    }

    pub fn is_possible(&self) -> bool {
        self.draw.iter().all(|draw| draw.is_possible())
    }
}