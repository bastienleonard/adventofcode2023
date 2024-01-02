use std::io::BufRead;

const VALUE_A: u32 = 14;
const VALUE_K: u32 = 13;
const VALUE_Q: u32 = 12;
const VALUE_J: u32 = 11;
const VALUE_T: u32 = 10;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Card {
    Value(u32),
    Joker
}

impl Card {
    fn require_value(self) -> u32 {
        match self {
            Self::Value(n) => n,
            Self::Joker => panic!("called require_value() on joker")
        }
    }
}

#[derive(Debug)]
struct Bid {
    cards: Vec<Card>,
    bid: u32
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum FixedUpCard {
    Regular { value: u32 },
    Joker { replaced_with: u32 }
}

impl FixedUpCard {
    fn value(self) -> u32 {
        match self {
            Self::Regular { value } => value,
            Self::Joker { replaced_with } => 1
        }
    }
}

#[derive(Debug)]
struct FixedUpBid {
    // TODO: use array or other immutable type?
    cards: Vec<FixedUpCard>,
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
    let bids = parse(lines);
    let mut bids: Vec<_> = bids.iter()
        .map(|bid| FixedUpBid {
            cards: bid.cards.iter()
                .map(|card| FixedUpCard::Regular {
                    value: card.require_value()
                })
                .collect(),
            bid: bid.bid        // TODO: avoid bid.bid
        })
        .collect();
    winnings(&mut bids)
}

pub fn part2() -> u32 {
    let file = std::fs::File::open("problem_inputs/7.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part2_result(&lines)
}

fn part2_result(lines: &[impl AsRef<str>]) -> u32 {
    let bids = parse(lines);
    let mut fixed_up_bids = replace_jokers(&bids);
    winnings(&mut fixed_up_bids)
}

fn parse(lines: &[impl AsRef<str>]) -> Vec<Bid> {
    lines.iter()
        .map(|line| parse_line(line.as_ref()))
        .collect()
}

fn parse_line(line: &str) -> Bid {
    let segments: Vec<_> = line.split_ascii_whitespace().collect();
    assert_eq!(segments.len(), 2);
    let cards: Vec<Card> = segments[0].chars()
        .map(parse_card)
        .collect();
    let bid = segments[1].parse::<u32>().unwrap();
    Bid { cards, bid }
}

fn parse_card(c: char) -> Card {
    let value = if let Some(n) = c.to_digit(10) {
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
    };

    Card::Value(value)
}

fn winnings(bids: &mut [FixedUpBid]) -> u32 {
    bids.sort_by(|a, b| compare_hands(&a.cards, &b.cards));
    bids.iter()
        .enumerate()
        // TODO: bid.bid is weird
        .map(|(i, bid)| bid.bid * (i as u32 + 1))
        .sum()
}

fn compare_hands(a: &[FixedUpCard], b: &[FixedUpCard]) -> std::cmp::Ordering {
    let hand_a = hand_type(a) as u8;
    let hand_b = hand_type(b) as u8;

    // TODO: simplify
    if hand_a < hand_b {
        std::cmp::Ordering::Less
    } else if hand_a > hand_b {
        std::cmp::Ordering::Greater
    } else {
        // TODO: performance
        a.iter()
            .map(|card| card.value())
            .collect::<Vec<u32>>()
            .cmp(
                &b.iter()
                    .map(|card| card.value())
                    .collect::<Vec<u32>>()
            )
    }
}

fn hand_type(cards: &[FixedUpCard]) -> HandType {
    let mut occurrences = std::collections::HashMap::<u32, u8>::new();

    for &card in cards {
        let key: u32 = match card {
            FixedUpCard::Regular { value } => value,
            FixedUpCard::Joker { replaced_with } => replaced_with
        };

        if let Some(n) = occurrences.get(&key) {
            occurrences.insert(key, n + 1);
        } else {
            occurrences.insert(key, 1);
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

// TODO: remove
// fn hand_type(cards: &[Card]) -> HandType {
//     let mut occurrences = std::collections::HashMap::<u32, u8>::new();

//     for &card in cards {
//         let key = card.require_value();

//         if let Some(n) = occurrences.get(&key) {
//             occurrences.insert(key, n + 1);
//         } else {
//             occurrences.insert(key, 1);
//         }
//     }

//     let values: Vec<u8> = occurrences.values().copied().collect();

//     if values.contains(&5) {
//         HandType::FiveOfAKind
//     } else if values.contains(&4) {
//         HandType::FourOfAKind
//     } else if values.contains(&3) && values.contains(&2) {
//         HandType::FullHouse
//     } else if values.contains(&3) {
//         HandType::ThreeOfAKind
//     } else if values.contains(&2) && values.len() == 3 {
//         HandType::TwoPair
//     } else if values.contains(&2) {
//         HandType::OnePair
//     } else {
//         assert!(values.iter().all(|&v| v == 1), "{:?}", values);
//         HandType::HighCard
//     }
// }

fn replace_jokers(bids: &[Bid]) -> Vec<FixedUpBid> {
    // TODO: review this mess
    bids.iter()
        .map(|bid| FixedUpBid {
            cards: replace_jokers_in_hand(&bid.cards),
            bid: bid.bid
        })
        .collect()
}

// TODO: receive a slice
// TODO: review this mess
fn replace_jokers_in_hand(cards: &Vec<Card>) -> Vec<FixedUpCard> {
    let mut occurrences = std::collections::HashMap::<Card, u8>::new();

    for &card in cards {
        if let Some(n) = occurrences.get(&card) {
            occurrences.insert(card, n + 1);
        } else {
            occurrences.insert(card, 1);
        }
    }

    if occurrences.get(&Card::Joker).copied().unwrap_or(0) == 0 {
        cards.iter()
            .map(|card| match card {
                Card::Joker => unreachable!(),
                Card::Value(n) => FixedUpCard::Regular { value: *n }
            })
            .collect()
    } else {
        let jokers_count: u8 = *(occurrences.get(&Card::Joker).unwrap());
        assert!(jokers_count > 0);

        if occurrences.len() == 1 {
            // All aces
            vec![FixedUpCard::Joker { replaced_with: VALUE_A }; 5]
        } else {
            let most_frequent_card: u32 =
                *(occurrences.iter()
                  .max_by_key(|(&key, &value): &(&Card, &u8)| match key {
                      Card::Joker => 0,
                      Card::Value(_) => value
                  })
                  .map(|(card, _)| match card {
                      Card::Joker => unreachable!(),
                      Card::Value(n) => n
                  })
                  .unwrap());
            cards.iter()
                .map(|&card| match card {
                    Card::Joker => FixedUpCard::Joker { replaced_with: most_frequent_card },
                    Card::Value(value) => FixedUpCard::Regular { value }
                })
                .collect()
        }
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

#[test]
fn test_part2() {
    let input = vec![
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483"
    ];
    assert_eq!(part2_result(&input), 5905);
}
