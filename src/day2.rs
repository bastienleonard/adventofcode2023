use std::io::BufRead;

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/2.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let games = lines.map(|line| parse_line(line.unwrap().as_str()));
    let mut result: u32 = 0;

    for (i, game) in games.enumerate() {
        if game.is_possible() {
            let game_id: u32 = (i + 1).try_into().unwrap();
            result += game_id;
        }
    }

    result
}

pub fn part2() -> u32 {
    let file = std::fs::File::open("problem_inputs/2.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let games = lines.map(|line| parse_line(line.unwrap().as_str()));
    games.map(|game| game.minimum_required().power()).sum()
}

#[derive(Debug, PartialEq)]
struct Game {
    sets: Vec<Cubes>
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|c| c.red <= 12 && c.green <= 13 && c.blue <= 14)
    }

    fn minimum_required(&self) -> Cubes {
        let mut max = Cubes { red: 0, green: 0, blue: 0 };

        for set in &self.sets {
            if set.red > max.red {
                max.red = set.red;
            }

            if set.green > max.green {
                max.green = set.green;
            }

            if set.blue > max.blue {
                max.blue = set.blue;
            }
        }

        max
    }
}

#[derive(Debug, PartialEq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32
}

impl Cubes {
    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_line(line: &str) -> Game {
    let sets = line.chars()
        .skip_while(|&c| c != ':')
        .skip(2)
        .collect::<String>();
    parse_sets(sets.as_str())
}

fn parse_sets(s: &str) -> Game {
    let split = s.split("; ");
    Game { sets: split.map(parse_set).collect() }
}

fn parse_set(s: &str) -> Cubes {
    let split: Vec<&str> = s.split(", ").collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for s in split {
        let x: Vec<_> = s.split(' ').collect();
        assert_eq!(x.len(), 2);
        let quantity = x[0].parse::<u32>().unwrap();
        let color = x[1];

        match color {
            "red" => {
                red = quantity;
            }
            "green" => {
                green = quantity;
            }
            "blue" => {
                blue = quantity;
            }
            _ => {
                unreachable!();
            }
        }
    }

    Cubes { red, green, blue }
}

#[test]
fn test_part1_parsing() {
    let data = [
        ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
         vec![Cubes { red: 4, green: 0, blue: 3 },
              Cubes { red: 1, green: 2, blue: 6 },
              Cubes { red: 0, green: 2, blue: 0 }]),
        ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
         vec![Cubes { red: 0, green: 2, blue: 1 },
              Cubes { red: 1, green: 3, blue: 4 },
              Cubes { red: 0, green: 1, blue: 1 }]),
        ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
         vec![Cubes { red: 20, green: 8, blue: 6 },
              Cubes { red: 4, green: 13, blue: 5 },
              Cubes { red: 1, green: 5, blue: 0 }]),
        ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
         vec![Cubes { red: 3, green: 1, blue: 6 },
              Cubes { red: 6, green: 3, blue: 0 },
              Cubes { red: 14, green: 3, blue: 15 }]),
        ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
         vec![Cubes { red: 6, green: 3, blue: 1 },
              Cubes { red: 1, green: 2, blue: 2 }])
    ];

    for (line, sets) in data {
        assert_eq!(parse_line(line), Game { sets });
    }
}
