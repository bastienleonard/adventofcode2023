use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Node {
    // TODO: rename to "id"?
    source: String,
    left: String,
    right: String
}

#[derive(Debug)]
struct Instructions {
    directions: Vec<Direction>,
    nodes: Vec<Node>
}

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/8.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part1_result(&lines)
}

fn part1_result(lines: &[impl AsRef<str>]) -> u32 {
    let instructions = parse_instructions(lines);
    let mut steps_count: u32 = 0;
    let mut direction_index: u32 = 0;
    let starting_node_index: u32 = find_aaa_index(&instructions);
    let mut current_node = &instructions.nodes[starting_node_index as usize];

    loop {
        if direction_index as usize == instructions.directions.len() {
            direction_index = 0;
        }

        let direction = instructions.directions[direction_index as usize];
        let next: &str = match direction {
            Direction::Left => &current_node.left,
            Direction::Right => &current_node.right
        };

        if next == "ZZZ" {
            return steps_count + 1;
        }

        current_node = instructions.nodes.iter()
            .find(|node| node.source == next)
            .unwrap();
        direction_index += 1;
        steps_count += 1;
    }
}

fn parse_instructions(lines: &[impl AsRef<str>]) -> Instructions {
    let directions = parse_directions(&lines[0]);
    let nodes = parse_nodes(&lines[2..]);
    Instructions { directions, nodes }
}

fn parse_directions(line: impl AsRef<str>) -> Vec<Direction> {
    line.as_ref()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!()
        }).collect()
}

fn parse_nodes(lines: &[impl AsRef<str>]) -> Vec<Node> {
    // TODO: don't use mut
    let mut result = Vec::new();

    for line in lines {
        let line = line.as_ref();
        let segments: Vec<&str> = line.split('=').collect();
        assert_eq!(segments.len(), 2);

        // TODO: performance
        let source: String = segments[0].trim().to_string();
        let (left, right) = parse_left_right(segments[1]);
        let node = Node { source, left, right };
        result.push(node);
    }

    result
}

fn parse_left_right(s: &str) -> (String, String) {
    let s = s.replace('(', "");
    let s = s.replace(')', "");
    let parts: Vec<&str> = s.split(", ").collect();
    assert_eq!(parts.len(), 2);

    // TODO: performance
    (parts[0].trim().to_string(), parts[1].to_string())
}

fn find_aaa_index(instructions: &Instructions) -> u32 {
    instructions.nodes.iter()
        .enumerate()
        .find(|(_i, node)| node.source == "AAA")
        .map(|(i, _node)| i.try_into().unwrap())
        .unwrap()
}

#[test]
fn test_part1_single_pass() {
    let input = vec![
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)"
    ];
    assert_eq!(part1_result(&input), 2);
}

#[test]
fn test_part1_several_passes() {
    let input = vec![
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)"
    ];
    assert_eq!(part1_result(&input), 6);
}
