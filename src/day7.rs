use std::io::BufRead;

#[derive(Debug)]
struct Bid {
    cards: Vec<u32>,
    bid: u32
}

#[derive(Copy, Clone)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/7.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part1_result(&lines)
}

fn part1_result(lines: &[impl AsRef<str>]) -> u32 {
    let mut bids = parse(lines);
    winnings(&mut bids)
}

fn parse(lines: &[impl AsRef<str>]) -> Vec<Bid> {
    lines.iter()
        .map(|line| parse_line(line.as_ref()))
        .collect()
}

fn parse_line(line: &str) -> Bid {
    let segments: Vec<_> = line.split_ascii_whitespace().collect();
    assert_eq!(segments.len(), 2);
    let cards: Vec<u32> = segments[0].chars()
        .map(parse_card_value)
        .collect();
    let bid = segments[1].parse::<u32>().unwrap();
    Bid { cards, bid }
}

fn parse_card_value(c: char) -> u32 {
    if let Some(n) = c.to_digit(10) {
        n
    } else {
        match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => {
                unreachable!("Unknown card {}", c);
            }
        }
    }
}

fn winnings(bids: &mut [Bid]) -> u32 {
    bids.sort_by(|a, b| compare_hands(&a.cards, &b.cards));
    bids.iter()
        .enumerate()
        // TODO: bid.bid is weird
        .map(|(i, bid)| bid.bid * (i as u32 + 1))
        .sum()
}

fn compare_hands(a: &[u32], b: &[u32]) -> std::cmp::Ordering {
    let hand_a = hand_type(a) as u8;
    let hand_b = hand_type(b) as u8;

    // TODO: simplify
    if hand_a < hand_b {
        std::cmp::Ordering::Less
    } else if hand_a > hand_b {
        std::cmp::Ordering::Greater
    } else {
        a.cmp(b)
    }
}

fn hand_type(cards: &[u32]) -> HandType {
    let mut occurrences = std::collections::HashMap::<u32, u8>::new();

    for &card in cards {
        if let Some(n) = occurrences.get(&card) {
            occurrences.insert(card, n + 1);
        } else {
            occurrences.insert(card, 1);
        }
    }

    let values: Vec<u8> = occurrences.values().copied().collect();

    if values.contains(&5) {
        HandType::FiveOfAKind
    } else if values.contains(&4) {
        HandType::FourOfAKind
    } else if values.contains(&3) && values.contains(&2) {
        HandType::FullHouse
    } else if values.contains(&3) {
        HandType::ThreeOfAKind
    } else if values.contains(&2) && values.len() == 3 {
        HandType::TwoPair
    } else if values.contains(&2) {
        HandType::OnePair
    } else {
        assert!(values.iter().all(|&v| v == 1), "{:?}", values);
        HandType::HighCard
    }
}

#[test]
fn test_part1() {
    let input = vec![
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483"
    ];
    assert_eq!(part1_result(&input), 6440);
}
