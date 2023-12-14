#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}
use std::fmt::Display;

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

fn part1(map: &[Vec<Tile>]) -> usize {
    tilt_north(map)
        .iter()
        .enumerate()
        .map(|(l_idx, line)| line.iter().filter(|t| t == &&RoundRock).count() * (map.len() - l_idx))
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    // print_map(&input);
    // print_map(&tilt_north(&input));
}
