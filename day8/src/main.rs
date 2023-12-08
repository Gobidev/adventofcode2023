use std::collections::HashMap;

use num::integer::lcm;

type Nodes = HashMap<String, (String, String)>;

fn node_from_line(line: &str) -> (String, (String, String)) {
    (
        line.chars().take(3).collect(),
        (
            line.chars().skip(7).take(3).collect(),
            line.chars().skip(12).take(3).collect(),
        ),
    )
}

fn parse(input_str: &str) -> (String, Nodes) {
    (
        input_str.lines().next().unwrap().to_string(),
        input_str.lines().skip(2).map(node_from_line).collect(),
    )
}

fn get_next_node(instruction: &char, current_node: &str, nodes: &Nodes) -> String {
    let children = nodes.get(current_node).unwrap();
    match instruction {
        'L' => children.0.clone(),
        'R' => children.1.clone(),
        _ => panic!(),
    }
}

fn part1(instructions: &str, nodes: &Nodes) -> u32 {
    let mut current_node = "AAA".to_string();
    let mut steps = 0;
    for instruction in instructions.chars().cycle() {
        if current_node == "ZZZ" {
            break;
        }
        current_node = get_next_node(&instruction, &current_node, nodes);
        steps += 1;
    }
    steps
}

fn part2(instructions: &str, nodes: &Nodes) -> u64 {
    nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|start_node| {
            let mut steps: u64 = 0;
            let mut current_node = start_node.clone();
            for instruction in instructions.chars().cycle() {
                if current_node.ends_with('Z') {
                    break;
                }
                current_node = get_next_node(&instruction, &current_node, nodes);
                steps += 1;
            }
            steps
        })
        .fold(1, lcm)
}

fn main() {
    let input = include_str!("../input.txt");
    let (instructions, nodes) = parse(input);
    println!("{}", part1(&instructions, &nodes));
    println!("{}", part2(&instructions, &nodes));
}
