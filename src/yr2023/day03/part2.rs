pub fn solve() {
    let input = include_str!("../input/day-03.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let parse_result = super::part1::parse(input);
    let ratios = parse_result.ratios;

    let mut sum = 0;

    for ratio in ratios.iter() {
        if ratio.1.len() == 2 {
            let prod = ratio.1[0] * ratio.1[1];
            sum += prod;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_day03() {
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

        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
