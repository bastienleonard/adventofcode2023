use std::io::BufRead;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    own: Vec<u32>
}

impl Card {
    fn matching_cards(&self) -> u32 {
        let mut result = 0u32;

        for n in &self.own {
            if self.winning.contains(n) {
                result += 1;
            }
        }

        result
    }

    fn points(&self) -> u32 {
        let mut result = 0u32;

        for n in &self.own {
            if self.winning.contains(n) {
                if result == 0 {
                    result = 1;
                } else {
                    result *= 2;
                }
            }
        }

        result
    }
}

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/4.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part1_result(lines)
}

pub fn part2() -> u32 {
    let file = std::fs::File::open("problem_inputs/4.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part2_result(lines)
}

fn part1_result(lines: Vec<String>) -> u32 {
    let cards = parse_cards(lines);
    cards.iter().map(|card| card.points()).sum()
}

fn part2_result(lines: Vec<String>) -> u32 {
    let cards = parse_cards(lines);
    let mut card_counts: Vec<u32> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let max_rewards: u32 = (cards.len() - i - 1).try_into().unwrap();
        let reward = std::cmp::min(card.matching_cards(), max_rewards);

        for _ in 0..card_counts[i] {
            for j in 0..reward {
                card_counts[i + ((j + 1) as usize)] += 1;
            }
        }
    }

    card_counts.iter().sum()
}

fn parse_cards(lines: Vec<String>) -> Vec<Card> {
    lines.iter().map(|line| parse_card(line)).collect()
}

fn parse_card(line: &str) -> Card {
    let line: String = line.chars()
        .skip_while(|&c| c != ':')
        .skip(1)
        .collect();
    let x: Vec<&str> = line.split('|').collect();
    assert_eq!(x.len(), 2);
    let winning: Vec<u32> = x[0].split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let own: Vec<u32> = x[1].split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    Card { winning, own }
}

#[test]
fn test_part1() {
    let data = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ];
    assert_eq!(
        part1_result(
            data.map(|line| line.to_owned()).to_vec()
        ),
        13
    );
}

#[test]
fn test_part2() {
    let data = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ];
    assert_eq!(
        part2_result(
            data.map(|line| line.to_owned()).to_vec()
        ),
        30
    );
}
