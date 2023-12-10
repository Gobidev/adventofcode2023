use std::collections::HashSet;

type Position = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    fn from_positions(old_pos: Position, new_pos: Position) -> Self {
        if old_pos.1 < new_pos.1 {
            East
        } else if old_pos.1 > new_pos.1 {
            West
        } else if old_pos.0 < new_pos.0 {
            South
        } else {
            North
        }
    }
    fn to_position(&self, old_pos: Position) -> Position {
        match self {
            North => (old_pos.0 - 1, old_pos.1),
            East => (old_pos.0, old_pos.1 + 1),
            South => (old_pos.0 + 1, old_pos.1),
            West => (old_pos.0, old_pos.1 - 1),
        }
    }
    fn invert(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Clone)]
struct Pipe(Vec<Direction>);

impl Pipe {
    fn from_char(c: char) -> Self {
        Self(match c {
            '|' => vec![North, South],
            '-' => vec![West, East],
            'L' => vec![North, East],
            'J' => vec![North, West],
            '7' => vec![South, West],
            'F' => vec![South, East],
            'S' => vec![North, East, South, West],
            _ => vec![],
        })
    }
}

fn parse(input_str: &str) -> (Vec<Vec<Pipe>>, Position) {
    let mut pipe_map: Vec<Vec<Pipe>> = input_str
        .lines()
        .map(|line| line.chars().map(Pipe::from_char).collect())
        .collect();
    // find start position
    let mut start_position = (0, 0);
    'outer: for (l_idx, l) in pipe_map.iter().enumerate() {
        for (r_idx, pipe) in l.iter().enumerate() {
            if pipe.0.len() == 4 {
                start_position = (l_idx, r_idx);
                break 'outer;
            }
        }
    }
    let mut start_pipe = vec![];
    if let Some(l) = pipe_map.get(start_position.0 - 1) {
        if let Some(p) = l.get(start_position.1) {
            if p.0.contains(&South) {
                start_pipe.push(North);
            }
        }
    }
    if let Some(l) = pipe_map.get(start_position.0 + 1) {
        if let Some(p) = l.get(start_position.1) {
            if p.0.contains(&North) {
                start_pipe.push(South);
            }
        }
    }
    if let Some(p) = pipe_map[start_position.0].get(start_position.1 - 1) {
        if p.0.contains(&East) {
            start_pipe.push(West);
        }
    }
    if let Some(p) = pipe_map[start_position.0].get(start_position.1 + 1) {
        if p.0.contains(&West) {
            start_pipe.push(East);
        }
    }
    // replace start position with actual pipe
    pipe_map[start_position.0][start_position.1] = Pipe(start_pipe);
    (pipe_map, start_position)
}

fn find_next_pos(pipe_map: &[Vec<Pipe>], prev_pos: Position, current_pos: Position) -> Position {
    let from_direction = Direction::from_positions(prev_pos, current_pos);
    pipe_map[current_pos.0][current_pos.1]
        .0
        .iter()
        .find(|dir| dir != &&from_direction.invert())
        .unwrap()
        .to_position(current_pos)
}

fn find_loop(pipe_map: &[Vec<Pipe>], start_pos: Position) -> Vec<Position> {
    let mut loop_positions: Vec<Position> = vec![start_pos];
    loop_positions.push(
        pipe_map[start_pos.0][start_pos.1]
            .0
            .first()
            .unwrap()
            .to_position(start_pos),
    );
    while loop_positions.last().unwrap() != &start_pos {
        loop_positions.push(find_next_pos(
            pipe_map,
            *loop_positions.iter().rev().nth(1).unwrap(),
            *loop_positions.last().unwrap(),
        ));
    }
    loop_positions
}

fn is_inside_loop(
    pipe_map: &[Vec<Pipe>],
    loop_positions: &HashSet<Position>,
    position: Position,
) -> bool {
    let mut crossings = 0;
    let mut current_pos = position;
    // we go up from the position and check how often we cross the loop
    while current_pos.0 != 0 {
        current_pos = (current_pos.0 - 1, current_pos.1);
        if !loop_positions.contains(&current_pos) {
            continue;
        }
        if pipe_map[current_pos.0][current_pos.1].0.contains(&East) {
            crossings += 1;
        }
    }
    crossings % 2 != 0
}

fn part1(pipe_map: &[Vec<Pipe>], start_pos: Position) -> usize {
    (find_loop(pipe_map, start_pos).len() - 1) / 2
}

fn part2(pipe_map: &[Vec<Pipe>], start_pos: Position) -> usize {
    let loop_positions: HashSet<Position> = HashSet::from_iter(find_loop(pipe_map, start_pos));
    let mut enclosed_tiles = 0;
    for pos1 in loop_positions.iter().map(|(x, _)| x).min().unwrap() + 1
        ..loop_positions.iter().map(|(x, _)| x).max().unwrap() - 1
    {
        for pos2 in loop_positions.iter().map(|(_, y)| y).min().unwrap() + 1
            ..loop_positions.iter().map(|(_, y)| y).max().unwrap() - 1
        {
            if loop_positions.contains(&(pos1, pos2)) {
                continue;
            }
            if is_inside_loop(pipe_map, &loop_positions, (pos1, pos2)) {
                enclosed_tiles += 1;
            }
        }
    }
    enclosed_tiles
}

fn main() {
    let (pipe_map, start_pos) = parse(include_str!("../input.txt"));
    println!("{}", part1(&pipe_map, start_pos));
    println!("{}", part2(&pipe_map, start_pos));
}
