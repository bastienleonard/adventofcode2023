use std::io::BufRead;

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/1.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    lines.map(|line| part1_parse_line(line.unwrap().as_str())).sum()
}

fn part1_parse_line(line: &str) -> u32 {
    let all_digits: Vec<u32> = line.chars()
        .filter_map(|char| char.to_digit(10))
        .collect();
    let result_digits: (u32, u32) = match all_digits.len() {
        1 => (all_digits[0], all_digits[0]),
        2.. => (all_digits[0], *all_digits.last().unwrap()),
        _ => unreachable!()
    };

    result_digits.0 * 10 + result_digits.1
}

#[test]
fn test_part1() {
    let data = [
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77)
    ];

    for (input, output) in data {
        assert_eq!(part1_parse_line(input), output, "{}", input);
    }
}

pub fn part2() -> u32 {
    let file = std::fs::File::open("problem_inputs/1.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    lines.map(|line| part2_parse_line(line.unwrap().as_str())).sum()
}

fn part2_parse_line(line: &str) -> u32 {
    let all_digits: Vec<u32> = part2_extract_all_digits(line);
    let result_digits: (u32, u32) = match all_digits.len() {
        1 => (all_digits[0], all_digits[0]),
        2.. => (all_digits[0], *all_digits.last().unwrap()),
        _ => unreachable!()
    };

    result_digits.0 * 10 + result_digits.1
}

fn part2_extract_all_digits(line: &str) -> Vec<u32> {
    let chars: Vec<char> = line.chars().collect();
    let mut result: Vec<u32> = Vec::new();
    let words = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    for (i, c) in chars.iter().enumerate() {
        match c.to_digit(10) {
            Some(digit) => {
                result.push(digit);
            }
            None => {
                for j in i..(chars.len()) {
                    let word = chars[i..=j].iter().collect::<String>();
                    let position = words.iter()
                        .position(|&x| x == word.as_str());

                    if let Some(position) = position {
                        result.push((position + 1).try_into().unwrap());
                    }
                }
            }
        }
    }

    result
}

#[test]
fn test_part2() {
    let data = [
        ("two1nine", 29),
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76)
    ];

    for (input, output) in data {
        assert_eq!(part2_parse_line(input), output, "{}", input);
    }
}
