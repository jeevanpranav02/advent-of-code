use super::parse;

pub fn solve() {
    // let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    //              Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    //              Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    //              Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    //              Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    //              Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let input = include_str!("../input/day-04.txt");
    dbg!(part1(input));
}

fn part1(input: &str) -> u32 {
    let cards = parse(input);
    let mut queue = (0..cards.len()).collect::<Vec<_>>();
    let mut visited: u32 = 0;

    while let Some(i) = queue.pop() {
        visited += 1;

        let card = &cards[i];
        if card.wins == 0 {
            continue;
        }

        for j in 0..card.wins as usize {
            queue.push(j + i + 1);
        }
    }

    visited.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_day04() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 30);
    }
}
