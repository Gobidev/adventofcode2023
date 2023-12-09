fn parse(input_str: &str) -> Vec<Vec<i64>> {
    input_str
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_history(sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut history: Vec<Vec<i64>> = vec![];
    let mut current_sequence = sequence.to_vec();
    while current_sequence.iter().any(|x| *x != 0) {
        history.push(current_sequence.clone());
        current_sequence = current_sequence.windows(2).map(|x| x[1] - x[0]).collect();
    }
    history
}

fn part1(parsed_input: &[Vec<i64>]) -> i64 {
    parsed_input
        .iter()
        .map(|sequence| {
            get_history(sequence)
                .iter()
                .map(|sequence| sequence.last().unwrap())
                .sum::<i64>()
        })
        .sum()
}

fn part2(parsed_input: &[Vec<i64>]) -> i64 {
    parsed_input
        .iter()
        .map(|sequence| {
            get_history(sequence)
                .iter()
                .enumerate()
                .map(|(idx, sequence)| {
                    sequence.first().unwrap() * if idx % 2 == 0 { 1 } else { -1 }
                })
                .sum::<i64>()
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
