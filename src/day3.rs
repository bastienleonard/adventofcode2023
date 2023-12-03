use std::io::BufRead;

struct Schematic {
    lines: Vec<String>
}

impl Schematic {
    fn get(&self, x: u32, y: u32) -> Option<char> {
        self.lines.get(y as usize)
            .and_then(|line| line.chars()
                      .nth(x as usize))
    }

    fn numbers(&self) -> Vec<NumberMatch> {
        let mut result = Vec::new();

        for (y, line) in self.lines.iter().enumerate() {
            let mut current_number_option: Option<NumberMatch> = None;

            for (x, c) in line.chars().enumerate() {
                let mut check = false;

                if let Some(digit) = c.to_digit(10) {
                    if let Some(number_match) = current_number_option {
                        current_number_option = Some(
                            NumberMatch {
                                x: number_match.x,
                                y: y as u32,
                                value: number_match.value * 10 + digit,
                                size: number_match.size + 1
                            }
                        );
                    } else {
                        current_number_option = Some(
                            NumberMatch {
                                x: x as u32,
                                y: y as u32,
                                value: digit,
                                size: 1
                            }
                        );
                    }
                } else {
                    check = true;
                }

                if x == line.len() - 1 {
                    check = true;
                }

                if check {
                    if let Some(number_match) = current_number_option {
                        result.push(number_match);
                        current_number_option = None;
                    }
                }
            }
        }

        result
    }

    fn stars(&self) -> Vec<StarMatch> {
        let mut result = Vec::new();

        for (y, line) in self.lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '*' {
                    result.push(
                        StarMatch {
                            x: x as u32,
                            y: y as u32
                        }
                    );
                }
            }
        }

        result
    }

    fn numbers_for_gear(&self, x: u32, y: u32) -> Vec<u32> {
        assert_eq!(self.get(x, y), Some('*'));

        self.numbers()
            .iter()
            .filter(|number|
                    ((number.x)..(number.x + number.size))
                    .any(|number_x: u32|
                         is_neighbor_of(number_x, number.y, x, y)))
            .map(|number| number.value)
            .collect()
    }
}

#[derive(Debug)]
struct NumberMatch {
    x: u32,
    y: u32,
    value: u32,
    size: u32
}

struct StarMatch {
    x: u32,
    y: u32
}

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/3.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part1_result(lines)
}

pub fn part2() -> u32 {
    let file = std::fs::File::open("problem_inputs/3.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part2_result(lines)
}

fn part1_result(lines: Vec<String>) -> u32 {
    let mut result = 0;
    let schematic = Schematic { lines };

    for number in schematic.numbers() {
        if is_next_to_symbol(
            number.x,
            number.y,
            number.size,
            &schematic
        ) {
            result += number.value;
        }
    }

    result
}

fn is_next_to_symbol(
    x: u32,
    y: u32,
    size: u32,
    schematic: &Schematic
) -> bool {
    assert!(size > 0);

    for x in x..(x + size) {
        let neighbors: [(i32, i32); 8] = [
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1)
        ];

        for (delta_x, delta_y) in neighbors {
            let neighbor_x: i32 = x as i32 + delta_x;
            let neighbor_y: i32 = y as i32 + delta_y;

            if neighbor_x >= 0 && neighbor_y >= 0 {
                let c = schematic.get(
                    neighbor_x as u32,
                    neighbor_y as u32
                );

                if let Some(c) = c {
                    if is_symbol(c) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn is_neighbor_of(x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
    let neighbor_deltas: [(i32, i32); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1)
    ];

    let x1: i32 = x1.try_into().unwrap();
    let y1: i32 = y1.try_into().unwrap();
    let x2: i32 = x2.try_into().unwrap();
    let y2: i32 = y2.try_into().unwrap();
    neighbor_deltas.iter()
        .any(|(delta_x, delta_y)| x1 + delta_x == x2 && y1 + delta_y == y2)
}

pub fn part2_result(lines: Vec<String>) -> u32 {
    let mut result = 0;
    let schematic = Schematic { lines };

    for star_match in schematic.stars() {
        let numbers = schematic.numbers_for_gear(star_match.x, star_match.y);

        if numbers.len() == 2 {
            result += numbers.iter().product::<u32>();
        }
    }

    result
}

#[test]
fn test_part1() {
    let data = [
        (vec!["123"], 0),
        (vec!["*123"], 123),
        (vec!["123*"], 123),
        (vec!["1*1"], 2),
        (vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        ],
         4361)
    ];

    for (input, output) in data {
        assert_eq!(
            part1_result(
                input.iter().map(|s| s.to_string()).collect()
            ),
            output
        );
    }
}

#[test]
fn test_part2() {
    let input = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
    ];
    assert_eq!(
        part2_result(
            input.iter().map(|s| s.to_string()).collect()
        ),
        467835
    );
}
