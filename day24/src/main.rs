#[derive(Debug, Clone)]
struct Hailstone {
    px: isize,
    py: isize,
    pz: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

type Position = (f64, f64);

impl Hailstone {
    fn from_line(line: &str) -> Self {
        let line = line.replace(',', "");
        let split: Vec<_> = line.split_whitespace().collect();
        Self {
            px: split[0].parse().unwrap(),
            py: split[1].parse().unwrap(),
            pz: split[2].parse().unwrap(),
            vx: split[4].parse().unwrap(),
            vy: split[5].parse().unwrap(),
            vz: split[6].parse().unwrap(),
        }
    }
    fn get_intersection(&self, other: &Hailstone) -> Option<Position> {
        let denominator = -self.vx * other.vy + other.vx * self.vy;
        if denominator == 0 {
            return None;
        }
        let t1 = (-(other.px - self.px) * other.vy + other.vx * (other.py - self.py)) as f64
            / denominator as f64;
        let t2 = ((other.py - self.py) * self.vx - self.vy * (other.px - self.px)) as f64
            / denominator as f64;
        let pos1 = (
            self.px as f64 + self.vx as f64 * t1,
            self.py as f64 + self.vy as f64 * t1,
        );
        if t1 >= 0. && t2 >= 0. {
            Some(pos1)
        } else {
            None
        }
    }
}

fn parse(input_str: &str) -> Vec<Hailstone> {
    input_str.lines().map(Hailstone::from_line).collect()
}

fn check_bounds(pos: &Position, lower_bound: f64, upper_bound: f64) -> bool {
    pos.0 >= lower_bound && pos.0 <= upper_bound && pos.1 >= lower_bound && pos.1 <= upper_bound
}

fn part1(hailstones: &[Hailstone]) -> usize {
    hailstones
        .iter()
        .enumerate()
        .map(|(h1_idx, h1)| {
            hailstones
                .iter()
                .skip(h1_idx)
                .map(|h2| {
                    if let Some(pos) = h1.get_intersection(h2) {
                        if check_bounds(&pos, 200000000000000., 400000000000000.) {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
}
