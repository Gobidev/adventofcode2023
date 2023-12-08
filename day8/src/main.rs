use std::collections::HashMap;

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
    let mut lines_iter = input_str.lines();
    let instructions = lines_iter.next().unwrap().to_string();
    let nodes: Nodes = lines_iter.skip(1).map(node_from_line).collect();
    (instructions, nodes)
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
    let mut current_node = nodes
        .keys()
        .find(|node| node.as_str() == "AAA")
        .unwrap()
        .clone();
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
    let start_nodes: Vec<String> = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect();
    start_nodes
        .iter()
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
        .fold(1, num::integer::lcm)
}

fn main() {
    let input = include_str!("../input.txt");
    let (instructions, nodes) = parse(input);
    println!("{}", part1(&instructions, &nodes));
    println!("{}", part2(&instructions, &nodes));
}
