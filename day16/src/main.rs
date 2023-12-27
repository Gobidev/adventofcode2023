use std::{fmt::Display, thread, time::Duration};
use rayon::prelude::*;

fn parse(input_str: &str) -> Vec<Vec<Tile>> {
    input_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '|' => Tile::new(NorthSouthSplitter),
                    '-' => Tile::new(WestEastSplitter),
                    '/' => Tile::new(UpMirror),
                    '\\' => Tile::new(DownMirror),
                    _ => Tile::new(Empty),
                })
                .collect()
        })
        .collect()
}

type Position = (isize, isize);

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    energized: Option<Direction>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tile_type {
            NorthSouthSplitter => write!(f, "|"),
            WestEastSplitter => write!(f, "-"),
            UpMirror => write!(f, "/"),
            DownMirror => write!(f, "\\"),
            Empty => {
                if let Some(d) = self.energized {
                    write!(f, "{d}")
                } else {
                    write!(f, ".")
                }
            }
        }
    }
}

impl Tile {
    fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            energized: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    NorthSouthSplitter,
    WestEastSplitter,
    UpMirror,
    DownMirror,
    Empty,
}
use TileType::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            North => write!(f, "^"),
            East => write!(f, ">"),
            South => write!(f, "v"),
            West => write!(f, "<"),
        }
    }
}

impl Direction {
    fn as_position(&self) -> Position {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn step_in_direction(&mut self) {
        let direction = self.direction.as_position();
        self.position = (
            (self.position.0 + direction.0),
            (self.position.1 + direction.1),
        );
    }

    fn move_beam(&mut self, map: &mut [Vec<Tile>]) -> Option<Beam> {
        map[self.position.0 as usize][self.position.1 as usize].energized = Some(self.direction);
        match map[self.position.0 as usize][self.position.1 as usize].tile_type {
            Empty => {
                self.step_in_direction();
                None
            }
            UpMirror => {
                match self.direction {
                    North => self.direction = East,
                    East => self.direction = North,
                    South => self.direction = West,
                    West => self.direction = South,
                }
                self.step_in_direction();
                None
            }
            DownMirror => {
                match self.direction {
                    North => self.direction = West,
                    East => self.direction = South,
                    South => self.direction = East,
                    West => self.direction = North,
                }
                self.step_in_direction();
                None
            }
            NorthSouthSplitter => {
                if matches!(self.direction, West | East) {
                    self.direction = North;
                    Some(Beam {
                        position: self.position,
                        direction: South,
                    })
                } else {
                    self.step_in_direction();
                    None
                }
            }
            WestEastSplitter => {
                if matches!(self.direction, North | South) {
                    self.direction = West;
                    Some(Beam {
                        position: self.position,
                        direction: East,
                    })
                } else {
                    self.step_in_direction();
                    None
                }
            }
        }
    }
}

fn print_map(map: &[Vec<Tile>]) {
    println!(
        "{}",
        map.iter()
            .map(|l| l.iter().map(|t| t.to_string()).collect::<String>() + "\n")
            .collect::<String>()
    )
}

const PRINT: bool = false;

fn calc_total_energized(map: &[Vec<Tile>], start_beam: Beam) -> usize {
    let mut map = map.to_vec();
    let mut beams = vec![start_beam];
    while !beams.is_empty() {
        if PRINT {
            print_map(&map);
            thread::sleep(Duration::from_millis(50));
        }
        // remove out of bounds beams and beam that travel on already travelled routes
        beams.retain(|beam| {
            0 <= beam.position.0
                && beam.position.0 < map.len() as isize
                && 0 <= beam.position.1
                && beam.position.1 < map[0].len() as isize
                && {
                    match map[beam.position.0 as usize][beam.position.1 as usize].energized {
                        None => true,
                        Some(d) => d != beam.direction,
                    }
                }
        });
        let mut new_beams = vec![];
        for beam in &mut beams {
            if let Some(b) = beam.move_beam(&mut map) {
                new_beams.push(b);
            }
        }
        beams.append(&mut new_beams);
    }
    map.iter()
        .map(|l| {
            l.iter()
                .map(|t| if t.energized.is_some() { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn part1(map: &[Vec<Tile>]) -> usize {
    calc_total_energized(
        map,
        Beam {
            position: (0, 0),
            direction: East,
        },
    )
}

fn part2(map: &[Vec<Tile>]) -> usize {
    let mut possible_start_beams = vec![];
    for l in 0..map.len() {
        possible_start_beams.push(Beam {
            position: (l as isize, 0),
            direction: East,
        });
        possible_start_beams.push(Beam {
            position: (l as isize, map[0].len() as isize - 1),
            direction: West,
        });
    }
    for c in 0..map[0].len() {
        possible_start_beams.push(Beam {
            position: (0, c as isize),
            direction: South,
        });
        possible_start_beams.push(Beam {
            position: (map.len() as isize - 1, c as isize),
            direction: North,
        });
    }
    possible_start_beams
        .par_iter()
        .map(|beam| calc_total_energized(map, beam.clone()))
        .max()
        .unwrap()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
