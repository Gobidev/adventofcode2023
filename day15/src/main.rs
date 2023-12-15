#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

fn parse(input_str: &str) -> Vec<String> {
    input_str
        .replace('\n', "")
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn hash(s: &str) -> u32 {
    let mut current_value = 0;
    for char in s.chars() {
        current_value += char as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn part1(input: &[String]) -> u32 {
    input.iter().map(|s| hash(s)).sum()
}

fn part2(input: &[String]) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for item in input {
        let label: String;
        let operation: char;
        let focal_length: usize;
        if item.ends_with('-') {
            operation = '-';
            label = item.chars().take(item.len() - 1).collect();
            focal_length = 0;
        } else {
            operation = '=';
            label = item.chars().take(item.len() - 2).collect();
            focal_length = item.chars().next_back().unwrap().to_digit(10).unwrap() as usize;
        }
        let box_idx = hash(&label) as usize;
        let lens = Lens {
            label,
            focal_length,
        };
        let target_lens = boxes[box_idx]
            .iter()
            .enumerate()
            .find(|(_, l)| l == &&lens)
            .map(|(idx, _)| idx);
        match operation {
            '-' => {
                if let Some(idx) = target_lens {
                    boxes[box_idx].remove(idx);
                }
            }
            _ => {
                if let Some(idx) = target_lens {
                    boxes[box_idx][idx] = lens;
                } else {
                    boxes[box_idx].push(lens);
                }
            }
        }
    }
    let mut sum = 0;
    for (box_idx, lens_box) in boxes.iter().enumerate() {
        for (lens_idx, lens) in lens_box.iter().enumerate() {
            sum += lens.focal_length * (lens_idx + 1) * (box_idx + 1);
        }
    }
    sum
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
