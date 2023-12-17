pub mod part1;
pub mod part2;

pub struct Card {
    wins: u8,
}

pub fn parse(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, scratch) = line.split_once(" | ").unwrap();
        let parse = |s: &str| {
            s.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u8>>()
        };

        let winning = parse(winning);
        let scratch = parse(scratch);
        cards.push(Card {
            wins: scratch.iter().filter(|x| winning.contains(x)).count() as u8,
        });
    }

    cards
}
