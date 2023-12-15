use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Operational,
    Damaged,
    Unknown,
}

type Cache = HashMap<(Vec<Tile>, Vec<u8>), usize>;

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operational => write!(f, "."),
            Damaged => write!(f, "#"),
            Unknown => write!(f, "?"),
        }
    }
}
use Tile::*;

fn parse(input_str: &str) -> Vec<(Vec<Tile>, Vec<u8>)> {
    input_str
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                split
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| match c {
                        '.' => Operational,
                        '#' => Damaged,
                        _ => Unknown,
                    })
                    .collect(),
                split
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|d| d.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn handle_operational(tiles: &[Tile], groups: &[u8], cache: &mut Cache) -> usize {
    calc_combinations(&tiles[1..], groups, cache)
}

fn handle_damaged(tiles: &[Tile], groups: &[u8], cache: &mut Cache) -> usize {
    let next_group_size = *groups.first().unwrap() as usize;
    if tiles
        .iter()
        .take(next_group_size)
        .filter(|t| t != &&Operational)
        .count()
        != next_group_size
    {
        return 0;
    }
    if tiles.len() == next_group_size {
        if groups.len() == 1 {
            return 1;
        }
        return 0;
    }
    if matches!(tiles[next_group_size], Unknown | Operational) {
        return calc_combinations(&tiles[next_group_size + 1..], &groups[1..], cache);
    }
    0
}

fn calc_combinations(tiles: &[Tile], groups: &[u8], cache: &mut Cache) -> usize {
    if groups.is_empty() {
        if !tiles.contains(&Damaged) {
            return 1;
        }
        return 0;
    }
    if tiles.is_empty() {
        return 0;
    }
    if let Some(val) = cache.get(&(tiles.to_vec(), groups.to_vec())) {
        return *val;
    }
    let res = match tiles.first().unwrap() {
        Operational => handle_operational(tiles, groups, cache),
        Damaged => handle_damaged(tiles, groups, cache),
        Unknown => handle_operational(tiles, groups, cache) + handle_damaged(tiles, groups, cache),
    };
    cache.insert((tiles.to_vec(), groups.to_vec()), res);
    res
}

fn part1(parsed_input: &[(Vec<Tile>, Vec<u8>)]) -> usize {
    let mut cache = HashMap::new();
    parsed_input
        .iter()
        .map(|(tiles, groups)| calc_combinations(tiles, groups, &mut cache))
        .sum()
}

fn part2(parsed_input: &[(Vec<Tile>, Vec<u8>)]) -> usize {
    let new_input: Vec<_> = parsed_input
        .iter()
        .map(|(tiles, groups)| {
            let mut new_tiles = tiles.clone();
            new_tiles.push(Unknown);
            new_tiles = new_tiles.repeat(5);
            new_tiles.pop();
            (new_tiles, groups.repeat(5))
        })
        .collect();
    let mut cache = HashMap::new();
    new_input
        .iter()
        .map(|(tiles, groups)| calc_combinations(tiles, groups, &mut cache))
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
