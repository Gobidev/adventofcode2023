fn parse(input_str: &str) -> Vec<Vec<String>> {
    input_str
        .split("\n\n")
        .map(|split| split.lines().map(|l| l.to_string()).collect())
        .collect()
}

fn check_mirror_correctness(block: &[String], lines_above: usize) -> bool {
    if lines_above <= block.len() / 2 {
        // mirror is in upper half -> check from top to bottom
        for i in 0..lines_above {
            if block[i] != block[2 * lines_above - i - 1] {
                return false;
            }
        }
        return true;
    } else {
        // mirror is in bottom half -> check from bottom to top
        for i in lines_above..block.len() - 1 {
            if block[i] != block[2 * lines_above - i - 1] {
                return false;
            }
        }
    }
    true
}

fn find_horizontal_mirror(block: &[String]) -> Option<usize> {
    for occurance in block
        .iter()
        .enumerate()
        .filter(|(_, l)| l == &&block[0])
        .map(|(idx, _)| idx)
        .skip(1)
    {
        let val = (occurance + occurance % 2) / 2;
        if check_mirror_correctness(block, val) {
            return Some(val);
        }
    }
    let last = block.last().unwrap();
    for occurance in block
        .iter()
        .enumerate()
        .filter(|(_, l)| l == &last)
        .map(|(idx, _)| idx)
        .rev()
        .skip(1)
    {
        let val = (block.len() - occurance + 1) / 2 + occurance;
        if check_mirror_correctness(block, val) {
            return Some(val);
        }
    }
    None
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

fn part1(blocks: &[Vec<String>]) -> usize {
    let mut sum = 0;
    for block in blocks {
        if let Some(n) = find_horizontal_mirror(block) {
            sum += 100 * n;
            continue;
        }
        if let Some(n) = find_horizontal_mirror(&transpose(block)) {
            sum += n;
        }
    }
    sum
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
}
