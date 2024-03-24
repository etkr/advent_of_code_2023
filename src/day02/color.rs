
#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    pub fn from_str(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => panic!("Invalid color"),
        }
    }
}