// src/days/day01/part1.rs

pub fn solve() {
    let input = include_str!("../input/day-01.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        if let (Some(&first_digit), Some(&last_digit)) = (digits.first(), digits.last()) {
            sum += first_digit as i32 * 10 + last_digit as i32;
        } else if let Some(&digit) = digits.first() {
            sum += digit as i32 * 10 + digit as i32;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample_input() {
        let input = "pqr3stu8vwx\n\
                     1abc2\n\
                     a1b2c3d4e5f\n\
                     treb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part1_empty() {
        let input = "";
        assert_eq!(part1(input), 0);
    }
}
