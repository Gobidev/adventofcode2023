#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
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
    fn from_color(col: u32) -> Self {
        match col & 15 {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
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

fn shoelace(positions: &[Position]) -> isize {
    let mut prev_point = positions.first().unwrap();
    let mut total_area: isize = 0;
    for position in positions.iter().skip(1) {
        total_area += (prev_point.0 + position.0) * (prev_point.1 - position.1);
        prev_point = position;
    }
    total_area.abs() / 2
}

fn calc_area(instructions: &[Instruction], part2: bool) -> isize {
    let mut corner_positions: Vec<Position> = vec![(0, 0)];
    let mut current_position = (0, 0);
    let mut perimiter = 0;
    let mut direction;
    let mut distance;
    for instruction in instructions {
        if part2 {
            direction = Direction::from_color(instruction.color).as_position();
            distance = (instruction.color >> 4) as isize;
        } else {
            direction = instruction.direction.as_position();
            distance = instruction.distance as isize;
        }
        perimiter += distance;
        current_position = (
            current_position.0 + distance * direction.0,
            current_position.1 + distance * direction.1,
        );
        corner_positions.push(current_position);
    }
    shoelace(&corner_positions) + perimiter / 2 + 1
}

fn part1(input: &[Instruction]) -> isize {
    calc_area(input, false)
}

fn part2(input: &[Instruction]) -> isize {
    calc_area(input, true)
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
