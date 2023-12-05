#[derive(Debug)]
struct Map {
    maps: Vec<(u32, u32, u32)>,
}

impl Map {
    fn from_block(block: &str) -> Self {
        Self {
            maps: block
                .lines()
                .skip(1)
                .map(|l| {
                    let mut val_iter = l.split(' ').map(|n| n.parse().unwrap());
                    (
                        val_iter.next().unwrap(),
                        val_iter.next().unwrap(),
                        val_iter.next().unwrap(),
                    )
                })
                .collect(),
        }
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
    (
        input
            .lines()
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect(),
        input.split("\n\n").skip(1).map(Map::from_block).collect(),
    )
}

fn map_to_end(seed: u32, maps: &[Map]) -> u32 {
    let mut current_val = seed;
    for map in maps {
        current_val = map.get_mapped_value(current_val);
    }
    current_val
}

fn part1(seeds: &[u32], maps: &[Map]) -> u32 {
    seeds
        .iter()
        .map(|seed| map_to_end(*seed, maps))
        .min()
        .unwrap()
}

fn part2(seeds: &[u32], maps: &[Map]) -> u32 {
    seeds
        .iter()
        .zip(seeds.iter().skip(1))
        .step_by(2)
        .flat_map(|(start, length)| *start..start + length)
        .map(|seed| map_to_end(seed, maps))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    let (seeds, maps) = parse(input);
    println!("{}", part1(&seeds, &maps));
    println!("{}", part2(&seeds, &maps));
}
