pub mod part1;
pub mod part2;

#[derive(Debug, Default)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

const MAX_CUBES: [u32; 3] = [12, 13, 14];

impl CubeSet {
    pub fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn is_possible(&self) -> bool {
        self.red <= MAX_CUBES[0] && self.green <= MAX_CUBES[1] && self.blue <= MAX_CUBES[2]
    }
}
