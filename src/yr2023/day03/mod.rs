pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Gear {
    value: u32,
    part_number: bool,
}

pub struct Result {
    gears: Vec<Gear>,
    ratios: std::collections::HashMap<(usize, usize), Vec<u32>>,
}
