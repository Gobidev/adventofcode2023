#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}
use std::{collections::HashMap, fmt::Display};

use Tile::*;

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoundRock => write!(f, "O"),
            CubeRock => write!(f, "#"),
            Empty => write!(f, "."),
        }
    }
}

fn parse(input_str: &str) -> Vec<Vec<Tile>> {
    input_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => RoundRock,
                    '#' => CubeRock,
                    _ => Empty,
                })
                .collect()
        })
        .collect()
}

fn print_map(map: &[Vec<Tile>]) {
    println!(
        "{}",
        map.iter()
            .map(|l| l.iter().map(|t| t.to_string()).collect::<String>() + "\n")
            .collect::<String>()
    );
}

fn tilt_north(map: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new_map = map.to_vec();
    for (l_idx, line) in map.iter().enumerate() {
        for (c_idx, tile) in line.iter().enumerate() {
            if tile == &RoundRock {
                let mut found = false;
                for i in (0..l_idx).rev() {
                    if matches!(new_map[i][c_idx], CubeRock | RoundRock) {
                        if l_idx != i + 1 {
                            new_map[i + 1][c_idx] = RoundRock;
                            new_map[l_idx][c_idx] = Empty;
                        }
                        found = true;
                        break;
                    }
                }
                if !found && l_idx != 0 {
                    new_map[0][c_idx] = RoundRock;
                    new_map[l_idx][c_idx] = Empty;
                }
            }
        }
    }
    new_map
}

fn transpose(map: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut res = vec![];
    for c in 0..map[0].len() {
        let mut row = vec![];
        for r in map {
            row.push(r[c].clone());
        }
        res.push(row);
    }
    res
}

fn reverse_lines(map: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new = map.to_vec();
    new.reverse();
    new
}

fn reverse_columns(map: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    map.to_vec()
        .iter_mut()
        .map(|l| {
            l.reverse();
            l.to_vec()
        })
        .collect()
}

fn north_beam_support(map: &[Vec<Tile>]) -> usize {
    map.iter()
        .enumerate()
        .map(|(l_idx, line)| line.iter().filter(|t| t == &&RoundRock).count() * (map.len() - l_idx))
        .sum()
}

fn part1(map: &[Vec<Tile>]) -> usize {
    north_beam_support(&tilt_north(map))
}

fn cycle_map(map: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    // north
    let mut current_map = tilt_north(map);
    // west
    current_map = transpose(&tilt_north(&transpose(&current_map)));
    // south
    current_map = reverse_lines(&tilt_north(&reverse_lines(&current_map)));
    // east
    current_map = reverse_columns(&transpose(&tilt_north(&transpose(&reverse_columns(
        &current_map,
    )))));
    current_map
}

fn part2(map: &[Vec<Tile>]) -> usize {
    let mut map_history: HashMap<Vec<Vec<Tile>>, usize> = HashMap::new();
    let mut current_map = map.to_vec();
    let mut count = 0;
    while map_history.get(&current_map).is_none() {
        map_history.insert(current_map.clone(), count);
        current_map = cycle_map(&current_map);
        count += 1;
    }
    let cycle_start = map_history.get(&current_map).unwrap();
    for _ in 0..(1_000_000_000 - cycle_start) % (count - cycle_start) {
        current_map = cycle_map(&current_map);
    }
    north_beam_support(&current_map)
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
