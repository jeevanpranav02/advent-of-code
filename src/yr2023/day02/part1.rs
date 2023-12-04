// Title: Day 02 Part 01

pub fn solve() {
    let input = include_str!("../input/day-02.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Default)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

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

const MAX_CUBES: [u32; 3] = [12, 13, 14];

fn part1(input: &str) -> u32 {
    let mut set = parse(input);

    let mut total = 0;

    for (i, games) in set.iter_mut().enumerate() {
        let mut max = CubeSet::default();
        for game in games.iter() {
            max = max.max(game);
        }

        if max.is_possible() {
            total += i + 1;
        }
    }

    total as u32
}

pub fn parse(input: &str) -> Vec<Vec<CubeSet>> {
    let mut sets = Vec::new();

    for line in input.lines() {
        let cubes = line.split_once(':').unwrap().1;

        let mut game_sets = Vec::new();
        for game in cubes.split(';') {
            let mut cubes = CubeSet::default();
            for i in game.split(',') {
                let mut iter = i.split_whitespace();
                let count = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap();

                match color {
                    "red" => cubes.red += count,
                    "green" => cubes.green += count,
                    "blue" => cubes.blue += count,
                    _ => unreachable!(),
                }
            }
            game_sets.push(cubes);
        }

        sets.push(game_sets);
    }
    return sets;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_day02() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part1(input);
        assert_eq!(output, 8);
    }
}
