use super::color::Color;

#[derive(PartialEq, Debug)]
pub struct Draw {
    pub quantity: i32,
    pub color: Color,
}

impl Draw {
    pub fn parse_vec(s: &str) -> Vec<Draw> {
        s.split(',').map(Draw::parse).collect()
    }

    pub fn parse(s: &str) -> Self {
        let f = s.split_whitespace().collect::<Vec<&str>>();
        let quantity = f[0].parse().unwrap();
        let color = Color::from_str(f[1]);
        Draw::new(quantity, color)
    }

    pub fn new(quantity: i32, color: Color) -> Self {
        Draw { quantity, color }
    }

    pub fn is_possible(&self) -> bool {
        match self {
            Draw {
                color: Color::Red,
                quantity,
            } if *quantity > 12 => false,
            Draw {
                color: Color::Blue,
                quantity,
            } if *quantity > 14 => false,
            Draw {
                color: Color::Green,
                quantity,
            } if *quantity > 13 => false,
            _ => true,
        }
    }
}
