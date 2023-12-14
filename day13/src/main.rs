fn parse(input_str: &str) -> Vec<Vec<String>> {
    input_str
        .split("\n\n")
        .map(|split| split.lines().map(|l| l.to_string()).collect())
        .collect()
}

fn diff(str1: &str, str2: &str) -> usize {
    str1.chars()
        .zip(str2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn check_mirror_correctness(block: &[String], lines_above: usize) -> usize {
    let mut errors = 0;
    if lines_above <= block.len() / 2 {
        for i in 0..lines_above {
            errors += diff(&block[i], &block[2 * lines_above - i - 1]);
        }
    } else {
        for i in lines_above..block.len() {
            errors += diff(&block[i], &block[2 * lines_above - i - 1]);
        }
    }
    errors
}

fn transpose(block: &[String]) -> Vec<String> {
    let mut res = vec![];
    for c in 0..block[0].len() {
        let mut row = "".to_string();
        for r in block {
            row.push(*r.as_bytes().get(c).unwrap() as char);
        }
        res.push(row);
    }
    res
}

fn mirror_sum(blocks: &[Vec<String>], error_count: usize) -> usize {
    let mut sum = 0;
    'outer: for block in blocks {
        for above in 1..block.len() {
            if check_mirror_correctness(block, above) == error_count {
                sum += 100 * above;
                continue 'outer;
            }
        }
        let block_t = transpose(block);
        for above in 1..block_t.len() {
            if check_mirror_correctness(&block_t, above) == error_count {
                sum += above;
            }
        }
    }
    sum
}

fn part1(blocks: &[Vec<String>]) -> usize {
    mirror_sum(blocks, 0)
}

fn part2(blocks: &[Vec<String>]) -> usize {
    mirror_sum(blocks, 1)
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
