type Position = (usize, usize);

fn parse_input(input_str: &str) -> Vec<Vec<u8>> {
    input_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn find_empty_spaces(map: &[Vec<u8>]) -> (Vec<usize>, Vec<usize>) {
    (
        map.iter()
            .enumerate()
            .filter(|(_, l)| l.iter().sum::<u8>() == 0)
            .map(|(idx, _)| idx)
            .collect(),
        {
            let mut col_indices = vec![];
            'outer: for c in 0..map[0].len() {
                for r in map {
                    if r[c] == 1 {
                        continue 'outer;
                    }
                }
                col_indices.push(c);
            }
            col_indices
        },
    )
}

fn galaxy_positions(map: &[Vec<u8>]) -> Vec<Position> {
    let mut res = vec![];
    for (l_idx, l) in map.iter().enumerate() {
        for (r_idx, r) in l.iter().enumerate() {
            if r == &1 {
                res.push((l_idx, r_idx));
            }
        }
    }
    res
}

fn expanded_galaxy_positions(map: &[Vec<u8>], expansion: usize) -> Vec<Position> {
    let (empty_rows, empty_columns) = find_empty_spaces(map);
    let mut new_positions = vec![];
    for position in galaxy_positions(map) {
        new_positions.push((
            position.0
                + empty_rows
                    .iter()
                    .filter(|r_idx| r_idx < &&position.0)
                    .count()
                    * expansion,
            position.1
                + empty_columns
                    .iter()
                    .filter(|c_idx| c_idx < &&position.1)
                    .count()
                    * expansion,
        ));
    }
    new_positions
}

fn all_galaxy_pairs(positions: &[Position]) -> Vec<(Position, Position)> {
    positions
        .iter()
        .enumerate()
        .flat_map(|(pos_idx, pos)| {
            positions
                .iter()
                .skip(pos_idx + 1)
                .map(|p| (*pos, *p))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn manhattan_distance(pos1: Position, pos2: Position) -> usize {
    ((pos1.0 as i64 - pos2.0 as i64).abs() + (pos1.1 as i64 - pos2.1 as i64).abs())
        .try_into()
        .unwrap()
}

fn total_distance(map: &[Vec<u8>], expansion: usize) -> usize {
    all_galaxy_pairs(&expanded_galaxy_positions(map, expansion))
        .iter()
        .map(|(pos1, pos2)| manhattan_distance(*pos1, *pos2))
        .sum()
}

fn part1(map: &[Vec<u8>]) -> usize {
    total_distance(map, 1)
}

fn part2(map: &[Vec<u8>]) -> usize {
    total_distance(map, 999999)
}

fn main() {
    let map = parse_input(include_str!("../input.txt"));
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}
