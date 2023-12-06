fn get_numbers_from_line(line: &str) -> Vec<u64> {
    line.split(' ')
        .skip(1)
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| num.parse().unwrap())
        .collect()
}

fn get_distance_for_hold_time(hold_time: u64, total_time: u64) -> u64 {
    (total_time - hold_time) * hold_time
}

fn part1(times: &[u64], distances: &[u64]) -> u64 {
    (0..times.len())
        .map(|i| {
            (1..times[i])
                .map(|hold_time| {
                    if get_distance_for_hold_time(hold_time, times[i]) > distances[i] {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .product()
}

fn concat_numbers(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .flat_map(|t| t.to_string().chars().collect::<Vec<_>>())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn part2(times: &[u64], distances: &[u64]) -> u64 {
    let time: u64 = concat_numbers(times);
    let distance: u64 = concat_numbers(distances);
    (1..time)
        .map(|hold_time| {
            if get_distance_for_hold_time(hold_time, time) > distance {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let mut input_lines = include_str!("../input.txt").lines();
    let times = get_numbers_from_line(input_lines.next().unwrap());
    let distances = get_numbers_from_line(input_lines.next().unwrap());
    println!("{}", part1(&times, &distances));
    println!("{}", part2(&times, &distances));
}
