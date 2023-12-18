#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use std::collections::HashSet;

use Direction::*;

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!(),
        }
    }
    fn as_position(&self) -> Position {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    distance: u32,
    color: u32,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let mut iter = line.split_whitespace();
        Self {
            direction: Direction::from_char(iter.next().unwrap().chars().next().unwrap()),
            distance: iter.next().unwrap().parse().unwrap(),
            color: u32::from_str_radix(
                &iter
                    .next()
                    .unwrap()
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect::<String>(),
                16,
            )
            .unwrap(),
        }
    }
}

type Position = (isize, isize);

fn parse(input_str: &str) -> Vec<Instruction> {
    input_str.lines().map(Instruction::from_line).collect()
}

fn instruction_positions(
    current_position: &Position,
    direction: &Direction,
    distance: u32,
) -> Vec<Position> {
    let mut new_positions = vec![];
    let direction_pos = direction.as_position();
    for i in 0..=distance {
        new_positions.push((
            current_position.0 + i as isize * direction_pos.0,
            current_position.1 + i as isize * direction_pos.1,
        ));
    }
    new_positions
}

fn positions_to_map(positions: &HashSet<Position>) -> Vec<Vec<char>> {
    let min_y = positions.iter().map(|(y, _)| y).min().unwrap();
    let max_y = positions.iter().map(|(y, _)| y).max().unwrap();
    let min_x = positions.iter().map(|(_, x)| x).min().unwrap();
    let max_x = positions.iter().map(|(_, x)| x).max().unwrap();
    let mut res = vec![];
    for y_idx in *min_y..=*max_y {
        let mut row = vec![];
        for x_idx in *min_x..=*max_x {
            if positions.contains(&(y_idx, x_idx)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        res.push(row);
    }
    res
}

fn fill_map(map: &mut [Vec<char>], pos: Position) {
    if map[pos.0 as usize][pos.1 as usize] == '.' {
        map[pos.0 as usize][pos.1 as usize] = '#';
        fill_map(map, (pos.0 + 1, pos.1));
        fill_map(map, (pos.0 - 1, pos.1));
        fill_map(map, (pos.0, pos.1 + 1));
        fill_map(map, (pos.0, pos.1 - 1));
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<char>]) {
    println!(
        "{}",
        map.iter()
            .map(|l| l.iter().collect::<String>() + "\n")
            .collect::<String>()
    );
}

fn part1(input: &[Instruction]) -> usize {
    let mut digged_positions: HashSet<Position> = HashSet::new();
    let mut current_position = (0, 0);
    for instruction in input {
        let new_positions = instruction_positions(
            &current_position,
            &instruction.direction,
            instruction.distance,
        );
        current_position = *new_positions.last().unwrap();
        digged_positions.extend(&mut new_positions.iter());
    }
    let mut map = positions_to_map(&digged_positions);
    let start_pos = map
        .get(1)
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, c)| c == &&'#')
        .map(|(i, _)| (1isize, (i + 1) as isize))
        .unwrap();
    fill_map(&mut map, start_pos);
    map.iter()
        .map(|l| {
            l.iter()
                .map(|c| if c == &'#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
}
