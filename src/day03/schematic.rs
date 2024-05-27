use nalgebra::DMatrix;

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

pub struct Schematic {
    pub matrix: DMatrix<char>,
}

impl Schematic {
    pub fn from_str(str: &[&str]) -> Schematic {
        let rows = str.len();
        let cols = str[0].len();
        let data: Vec<char> = str.iter().flat_map(|s| s.chars()).collect();
        let matrix = DMatrix::from_row_slice(rows, cols, &data);
        Schematic::new(matrix)
    }

    fn new(matrix: DMatrix<char>) -> Self {
        Schematic { matrix }
    }

    pub fn sum_of_part_numbers(&self) -> i32 {
        self.find_part_numbers().iter().sum()
    }

    pub fn has_adjecent_symbol(&self, c: usize, r: usize) -> bool {
        let mut result = false;
        if r > 0 {
            result = result || is_symbol(self.matrix[(c, r - 1)]);
        }
        if r < self.matrix.nrows() - 1 {
            result = result || is_symbol(self.matrix[(c, r + 1)]);
        }
        if c > 0 {
            result = result || is_symbol(self.matrix[(c - 1, r)]);
        }
        if c < self.matrix.ncols() - 1 {
            result = result || is_symbol(self.matrix[(c + 1, r)]);
        }
        if c > 0 && r > 0 {
            result = result || is_symbol(self.matrix[(c - 1, r - 1)]);
        }
        if c < self.matrix.ncols() - 1 && r < self.matrix.nrows() - 1 {
            result = result || is_symbol(self.matrix[(c + 1, r + 1)]);
        }
        if c > 0 && r < self.matrix.nrows() - 1 {
            result = result || is_symbol(self.matrix[(c - 1, r + 1)]);
        }
        if c < self.matrix.ncols() - 1 && r > 0 {
            result = result || is_symbol(self.matrix[(c + 1, r - 1)]);
        }
        result
    }

    pub fn find_part_numbers(&self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for c in 0..self.matrix.ncols() {
            let mut buffer: Vec<char> = Vec::new();
            let mut has_symbol = false;

            for r in 0..self.matrix.nrows() {
                let char = self.matrix[(c, r)];
                if char.is_ascii_digit() {
                    if self.has_adjecent_symbol(c, r) {
                        has_symbol = true;
                    }

                    buffer.push(char);
                } else if !buffer.is_empty() && has_symbol {
                    let s = buffer.iter().collect::<String>();
                    let n = s.parse::<i32>().unwrap();
                    result.push(n);
                    buffer.clear();
                    has_symbol = false;
                } else {
                    buffer.clear();
                    has_symbol = false;
                }
            }
        }
        result
    }
}
