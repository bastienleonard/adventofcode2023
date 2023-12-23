use std::io::BufRead;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum CardValue {
    A,
    K,
    Q,
    // J,
    T,
    Number(u32)
}

impl CardValue {
    fn value(self) -> u32 {
        match self {
            Self::A => 14,
            Self::K => 13,
            Self::Q => 12,
            // Self::J => 11,
            Self::T => 10,
            Self::Number(n) => n
        }
    }
}

impl std::fmt::Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match *self {
            Self::A => 'A',
            Self::K => 'K',
            Self::Q => 'Q',
            Self::T => 'T',
            Self::Number(n) => char::from_digit(n, 10).unwrap()
        };
        write!(f, "{}", c)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum InitialCard {
    Value(CardValue),
    Joker
}

impl std::fmt::Display for InitialCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Value(value) => write!(f, "{}", value),
            Self::Joker => write!(f, "J")
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum FixedUpCard {
    Regular { value: CardValue },
    Joker { replaced_with: CardValue }
}

impl FixedUpCard {
    fn value(self) -> u32 {
        match self {
            Self::Regular { value } => value.value(),
            Self::Joker { replaced_with: _ } => 1
        }
    }
}

impl std::fmt::Display for FixedUpCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Regular { value } => {
                write!(f, "{}", value)?;
            },
            Self::Joker { replaced_with } => {
                write!(f, "{}", replaced_with)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Bid {
    cards: Vec<InitialCard>,
    value: u32
}

impl std::fmt::Display for Bid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }

        write!(f, " {}", self.value)
    }
}

#[derive(Debug)]
struct FixedUpBid {
    cards: Vec<FixedUpCard>,
    value: u32
}

impl std::fmt::Display for FixedUpBid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }

        write!(f, " {}", self.value)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
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
    let bids_with_jokers = parse(lines);
    let mut bids = replace_jokers(&bids_with_jokers);
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
    let cards: Vec<InitialCard> = segments[0].chars()
        .map(parse_card)
        .collect();
    let value = segments[1].parse::<u32>().unwrap();
    Bid { cards, value }
}

fn parse_card(c: char) -> InitialCard {
    if let Some(n) = c.to_digit(10) {
        InitialCard::Value(CardValue::Number(n))
    } else {
        match c {
            'T' => InitialCard::Value(CardValue::T),
            'J' => InitialCard::Joker,
            'Q' => InitialCard::Value(CardValue::Q),
            'K' => InitialCard::Value(CardValue::K),
            'A' => InitialCard::Value(CardValue::A),
            _ => {
                unreachable!("Unknown card {}", c);
            }
        }
    }
}

fn winnings(bids: &mut [FixedUpBid]) -> u32 {
    bids.sort_by(|a, b| compare_hands(&a.cards, &b.cards));
    bids.iter()
        .enumerate()
        .map(|(i, bid)| bid.value * (i as u32 + 1))
        .sum()
}

fn compare_hands(a: &[FixedUpCard], b: &[FixedUpCard]) -> std::cmp::Ordering {
    let hand_a = hand_type(a) as u8;
    let hand_b = hand_type(b) as u8;

    #[allow(clippy::comparison_chain)]
    if hand_a < hand_b {
        std::cmp::Ordering::Less
    } else if hand_a > hand_b {
        std::cmp::Ordering::Greater
    } else {
        a.iter()
            .map(|card| card.value())
            .cmp(b.iter().map(|card| card.value()))
    }
}

fn hand_type(cards: &[FixedUpCard]) -> HandType {
    let mut occurrences = std::collections::HashMap::<u32, u8>::new();

    for &card in cards {
        let key: u32 = match card {
            FixedUpCard::Regular { value } => value.value(),
            FixedUpCard::Joker { replaced_with } => replaced_with.value()
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

fn replace_jokers(bids: &[Bid]) -> Vec<FixedUpBid> {
    bids.iter()
        .map(|bid| FixedUpBid {
            cards: replace_jokers_in_hand(&bid.cards),
            value: bid.value
        })
        .collect()
}

fn replace_jokers_in_hand(cards: &Vec<InitialCard>) -> Vec<FixedUpCard> {
    let mut occurrences = std::collections::HashMap::<InitialCard, u8>::new();

    for &card in cards {
        if let Some(n) = occurrences.get(&card) {
            occurrences.insert(card, n + 1);
        } else {
            occurrences.insert(card, 1);
        }
    }

    if occurrences.get(&InitialCard::Joker).copied().unwrap_or(0) == 0 {
        cards.iter()
            .map(|card| match card {
                InitialCard::Joker => unreachable!(),
                InitialCard::Value(n) => FixedUpCard::Regular { value: *n }
            })
            .collect()
    } else {
        let jokers_count: u8 = *(occurrences.get(&InitialCard::Joker).unwrap());
        assert!(jokers_count > 0);

        if occurrences.len() == 1 {
            // All aces
            vec![FixedUpCard::Joker { replaced_with: CardValue::A }; 5]
        } else {
            let most_frequent_card: CardValue =
                *(occurrences.iter()
                  .max_by_key(|(&key, &value): &(&InitialCard, &u8)| match key {
                      InitialCard::Joker => 0,
                      InitialCard::Value(_) => value
                  })
                  .map(|(card, _)| match card {
                      InitialCard::Joker => unreachable!(),
                      InitialCard::Value(n) => n
                  })
                  .unwrap());
            cards.iter()
                .map(|&card| match card {
                    InitialCard::Joker => FixedUpCard::Joker { replaced_with: most_frequent_card },
                    InitialCard::Value(value) => FixedUpCard::Regular { value }
                })
                .collect()
        }
    }
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
