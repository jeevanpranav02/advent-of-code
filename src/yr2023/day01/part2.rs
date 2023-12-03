// src/days/day01/part2.rs

pub fn solve() {
    let input = include_str!("../input/day-01.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for i in input.lines() {
        let mut first = None;
        let mut last = 0;

        let mut digit = |c| {
            first = first.or(Some(c));
            last = c;
        };

        let chars = i.as_bytes();
        for (i, c) in chars.iter().enumerate() {
            if c.is_ascii_digit() {
                digit((c - b'0') as u32);
            } else {
                for (j, d) in DIGITS.iter().enumerate() {
                    if chars[i..].starts_with(d.as_bytes()) {
                        digit(j as u32 + 1);
                    }
                }
            }
        }

        sum += first.unwrap() * 10 + last;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_for_given_input() {
        let input = "two1nine\n\
                     eightwothree\n\
                     abcone2threexyz\n\
                     xtwone3four\n\
                     4nineeightseven2\n\
                     zoneight234\n\
                     7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }
}
