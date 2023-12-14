use std::io::BufRead;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>
}

#[derive(Debug)]
struct Map {
    from: Resource,
    to: Resource,
    ranges: Vec<MapRange>
}

#[derive(Debug, Copy, Clone)]
struct MapRange {
    destination_start: u32,
    source_start: u32,
    length: u32
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

pub fn part1() -> u32 {
    let file = std::fs::File::open("problem_inputs/5.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part1_result(&lines)
}

fn part1_result<T: AsRef<str>>(lines: &[T]) -> u32 {
    let almanac = parse_almanac(lines);
    almanac.seeds.iter()
        .map(|&seed| seed_location(seed, &almanac))
        .min()
        .unwrap()
}

pub fn part2() -> u32 {
    let file = std::fs::File::open("problem_inputs/5.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    part2_result(&lines)
}

fn part2_result<T: AsRef<str>>(lines: &[T]) -> u32 {
    let almanac = parse_almanac(lines);
    let mut result = u32::MAX;

    for i in (0..almanac.seeds.len()).step_by(2) {
        let range_start = almanac.seeds[i];
        let range_length = almanac.seeds[i + 1];
        let mut seed = range_start;

        for _ in 0..range_length {
            let location = seed_location(seed, &almanac);

            if location < result {
                result = location;
            }

            seed += 1;
        }
    }

    result
}

fn seed_location(seed: u32, almanac: &Almanac) -> u32 {
    let mut resource = Resource::Seed;
    let mut id = seed;

    fn range_contains(range: MapRange, source: u32) -> bool {
        let source_start = range.source_start as u64;
        let length = range.length as u64;
        let source = source as u64;
        (source_start..(source_start + length)).contains(&source)
    }

    for map in &almanac.maps {
        assert_eq!(map.from, resource);
        let ranges: Vec<MapRange> = map.ranges.iter()
            .filter(|&&range| range_contains(range, id))
            .copied()
            .collect();
        assert!(ranges.len() <= 1, "{} {:?}", id, ranges);

        if let Some(range) = ranges.first() {
            id = range.destination_start + (id - range.source_start);
        }

        resource = map.to;
    }

    assert_eq!(resource, Resource::Location);
    id
}

fn parse_almanac<T: AsRef<str>>(lines: &[T]) -> Almanac {
    let seeds = parse_seeds(&lines[0]);
    let maps = parse_maps(&lines[2..]);
    Almanac { seeds, maps }
}

fn parse_seeds<T: AsRef<str>>(line: T) -> Vec<u32> {
    line.as_ref()
        .strip_prefix("seeds: ").unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn parse_maps<T: AsRef<str>>(lines: &[T]) -> Vec<Map> {
    let mut result = vec![];
    let mut resources: Option<(Resource, Resource)> = None;
    let mut ranges: Vec<MapRange> = Vec::new();

    for line in lines {
        let line = line.as_ref();

        if let Some((from, to)) = resources {
            if line.is_empty() {
                result.push(Map { from, to, ranges });
                resources = None;
                ranges = Vec::new();
            } else {
                ranges.push(parse_range(line));
            }
        } else {
            let (from, to) = parse_resources(line);
            resources = Some((from, to));
        }
    }

    if let Some((from, to)) = resources {
        result.push(Map { from, to, ranges});
    }

    result
}

fn parse_resources(line: &str) -> (Resource, Resource) {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();
    assert_eq!(parts.len(), 2, "{:?}", parts);
    assert_eq!(parts[1], "map:");
    let parts: Vec<&str> = parts[0].split('-').collect();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[1], "to");
    let from = parse_resource(parts[0]);
    let to = parse_resource(parts[2]);
    (from, to)
}

fn parse_range(line: &str) -> MapRange {
    let numbers: Vec<u32> = line.split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);
    MapRange {
        destination_start: numbers[0],
        source_start: numbers[1],
        length: numbers[2]
    }
}

fn parse_resource(s: &str) -> Resource {
    match s {
        "soil" => Resource::Soil,
        "seed" => Resource::Seed,
        "fertilizer" => Resource::Fertilizer,
        "water" => Resource::Water,
        "light" => Resource::Light,
        "temperature" => Resource::Temperature,
        "humidity" => Resource::Humidity,
        "location" => Resource::Location,
        _ => {
            panic!("Unknown resource {:?}", s);
        }
    }
}

#[test]
fn test_part1() {
    let input = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];
    assert_eq!(part1_result(&input), 35);
}

#[test]
fn test_part2() {
    let input = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];
    assert_eq!(part2_result(&input), 46);
}
