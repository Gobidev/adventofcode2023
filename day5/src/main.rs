#[derive(Debug)]
struct Map {
    maps: Vec<(u32, u32, u32)>,
}

impl Map {
    fn new() -> Self {
        Self {
            maps: vec![],
        }
    }
    fn from_block(block: &str) -> Self {
        let mut result = Map::new();
        for range_str in block.lines().skip(1) {
            let range_values: Vec<u32> = range_str.split(' ').map(|n| n.parse().unwrap()).collect();
            result
                .maps
                .push((range_values[0], range_values[1], range_values[2]))
        }
        result
    }
    fn get_mapped_value(&self, src: u32) -> u32 {
        for map in &self.maps {
            if src >= map.1 && src <= map.1 + map.2 {
                return map.0 + (src - map.1);
            }
        }
        src
    }
}

fn parse(input: &str) -> (Vec<u32>, Vec<Map>) {
    let seeds: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut maps: Vec<Map> = vec![];
    for block in input.split("\n\n").skip(1) {
        maps.push(Map::from_block(block));
    }
    (seeds, maps)
}

fn part1(seeds: &[u32], maps: &[Map]) -> u32 {
    let mut current_min = u32::MAX;
    for seed in seeds {
        let mut current_val = *seed;
        for map in maps {
            current_val = map.get_mapped_value(current_val);
        }
        if current_val < current_min {
            current_min = current_val;
        }
    }
    current_min
}

fn part2(seeds: &[u32], maps: &[Map]) -> u32 {
    let seed_ranges = seeds
        .iter()
        .zip(seeds.iter().skip(1))
        .step_by(2)
        .map(|(start, length)| *start..start + length);
    let mut current_min = u32::MAX;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let mut current_val = seed;
            for map in maps {
                current_val = map.get_mapped_value(current_val);
            }
            if current_val < current_min {
                current_min = current_val;
            }
        }
    }
    current_min
}

fn main() {
    let input = include_str!("../input.txt");
    let (seeds, maps) = parse(input);
    println!("{}", part1(&seeds, &maps));
    println!("{}", part2(&seeds, &maps));
}
