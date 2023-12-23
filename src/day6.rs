use std::io::BufRead;

#[derive(Copy, Clone)]
struct Race {
    duration: u64,
    best_distance: u64
}

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/6.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part1_result(&lines)
}

pub fn part2() -> u32 {
    let file = std::fs::File::open("problem_inputs/6.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part2_result(&lines)
}

fn part1_result<T: AsRef<str>>(lines: &[T]) -> u32 {
    let lines: Vec<_> = lines.iter()
        .map(|line| part1_parse_line(line.as_ref()))
        .collect();
    assert_eq!(lines.len(), 2);
    lines[0].iter()
        .zip(&lines[1])
        .map(|(&duration, &best_distance)| Race { duration, best_distance })
        .map(win_possibilities)
        .product()
}

fn part1_parse_line(s: &str) -> Vec<u64> {
    let i = s.find(':').unwrap();
    let s = &s[(i + 1)..];
    s.trim()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn win_possibilities(race: Race) -> u32 {
    (1..(race.duration))
        .filter(|&pressed_duration| leads_to_win(pressed_duration, race))
        .count()
        .try_into()
        .unwrap()
}

fn leads_to_win(pressed_duration: u64, race: Race) -> bool {
    assert_ne!(pressed_duration, 0);
    assert_ne!(pressed_duration, race.duration);
    let speed = pressed_duration;
    let distance = speed * (race.duration - pressed_duration);
    distance > race.best_distance
}

fn part2_result<T: AsRef<str>>(lines: &[T]) -> u32 {
    let numbers: Vec<_> = lines.iter()
        .map(|line| part2_parse_line(line.as_ref()))
        .collect();
    assert_eq!(numbers.len(), 2);
    let race = Race {
        duration: numbers[0],
        best_distance: numbers[1]
    };
    win_possibilities(race)
}

fn part2_parse_line(s: &str) -> u64 {
    let i = s.find(':').unwrap();
    let s = s[(i + 1)..].trim();
    let s: String = s.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    s.parse().unwrap()
}

#[test]
fn test_part1() {
    let input = vec![
        "Time:      7  15   30",
        "Distance:  9  40  200"
    ];
    assert_eq!(part1_result(&input), 288);
}

#[test]
fn test_part2() {
    let input = vec![
        "Time:      7  15   30",
        "Distance:  9  40  200"
    ];
    assert_eq!(part2_result(&input), 71503);
}
