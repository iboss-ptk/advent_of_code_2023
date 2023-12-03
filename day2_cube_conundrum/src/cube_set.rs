#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Default, Debug, Clone)]
pub struct CubeSet {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl From<Vec<(u64, Color)>> for CubeSet {
    fn from(count_color_pairs: Vec<(u64, Color)>) -> Self {
        let mut cube_set = CubeSet::default();
        for (count, color) in count_color_pairs {
            match color {
                Color::Red => cube_set.red += count,
                Color::Green => cube_set.green += count,
                Color::Blue => cube_set.blue += count,
            }
        }
        cube_set
    }
}

impl CubeSet {
    pub fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

pub const BAG: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};
