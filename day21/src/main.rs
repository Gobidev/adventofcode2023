#[derive(Debug, PartialEq)]
enum Tile {
    Start,
    Plot,
    Rock,
}
use std::collections::HashSet;

use Tile::*;

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Start,
            '#' => Rock,
            _ => Plot,
        }
    }
}

fn parse(input_str: &str) -> Vec<Vec<Tile>> {
    input_str
        .lines()
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect()
}

type Position = (usize, usize);

fn get_neighboring_plots(map: &[Vec<Tile>], pos: Position) -> Vec<Position> {
    let mut res = vec![];
    if let Some(row) = map.get(pos.0.wrapping_sub(1)) {
        if let Some(t) = row.get(pos.1) {
            if t != &Rock {
                res.push((pos.0 - 1, pos.1));
            }
        }
    }
    if let Some(row) = map.get(pos.0 + 1) {
        if let Some(t) = row.get(pos.1) {
            if t != &Rock {
                res.push((pos.0 + 1, pos.1));
            }
        }
    }
    if let Some(row) = map.get(pos.0) {
        if let Some(t) = row.get(pos.1.wrapping_sub(1)) {
            if t != &Rock {
                res.push((pos.0, pos.1 - 1));
            }
        }
    }
    if let Some(row) = map.get(pos.0) {
        if let Some(t) = row.get(pos.1 + 1) {
            if t != &Rock {
                res.push((pos.0, pos.1 + 1));
            }
        }
    }
    res
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Tile>], positions: &HashSet<Position>) {
    for (r_idx, r) in map.iter().enumerate() {
        for (t_idx, t) in r.iter().enumerate() {
            if positions.contains(&(r_idx, t_idx)) {
                print!("O");
                continue;
            }
            match t {
                Rock => print!("#"),
                Start => print!("S"),
                Plot => print!("."),
            }
        }
        println!();
    }
}

fn part1(map: &[Vec<Tile>], iterations: usize) -> usize {
    let mut current_positions = HashSet::new();
    'outer: for (r_idx, r) in map.iter().enumerate() {
        for (t_idx, t) in r.iter().enumerate() {
            if t == &Start {
                current_positions.insert((r_idx, t_idx));
                break 'outer;
            }
        }
    }
    for _ in 0..iterations {
        let mut new_positions = HashSet::new();
        for pos in &current_positions {
            let neighboring = get_neighboring_plots(map, *pos);
            new_positions.extend(&mut neighboring.iter());
        }
        current_positions = new_positions;
    }
    current_positions.len()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input, 64));
}
