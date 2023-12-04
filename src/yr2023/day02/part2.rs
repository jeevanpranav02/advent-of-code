// Title: Day 02 Part 2

use crate::yr2023::day02::part1::CubeSet;

pub fn solve() {
    let input = include_str!("../input/day-02.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut set = super::part1::parse(input);

    let mut total = 0;

    for (_, games) in set.iter_mut().enumerate() {
        let mut max = CubeSet::default();
        for game in games.iter() {
            max = max.max(game);
        }

        total += max.red * max.green * max.blue;
    }

    total as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_day02() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part2(input);
        assert_eq!(output, 2286);
    }
}
