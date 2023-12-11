use super::{Gear, Result};
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../input/day-03.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn parse(input: &str) -> Result {
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.insert((x, y), c);
            }
        }
    }

    let mut gears = Vec::new();
    let mut ratios = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for m in Regex::new(r"\d+").unwrap().find_iter(line) {
            let value = m.as_str().parse().unwrap();

            let mut part_number = false;
            for nx in m.start().saturating_sub(1)..=m.end() {
                for ny in y.saturating_sub(1)..=y + 1 {
                    let pos = (nx, ny);
                    let symbol = symbols.get(&pos);
                    part_number |= symbol.is_some();

                    if symbol == Some(&'*') {
                        ratios.entry(pos).or_insert(Vec::new()).push(value);
                    }
                }
            }

            gears.push(Gear { value, part_number });
        }
    }

    Result { gears, ratios }
}
fn part1(input: &str) -> i32 {
    let parse_result = parse(input);
    let gears = parse_result.gears;
    let mut sum = 0;

    for (value, part_number) in gears.iter().map(|gear| (gear.value, gear.part_number)) {
        if part_number {
            sum += value as i32;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_day03() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";

        let result = part1(input);
        assert_eq!(result, 4361);
    }
}
