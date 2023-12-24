use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
struct Part {
    values: HashMap<Feature, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Feature {
    X,
    M,
    A,
    S,
}

impl Feature {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    target_feature: Feature,
    operator: char,
    threshold: usize,
    target_workflow: String,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    final_workflow: String,
}

lazy_static! {
    static ref CONDITION_REGEX: Regex = Regex::new(r"([xmas])(<|>)(\d*):(\w*),").unwrap();
}

fn parse(input_str: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut split = input_str.split("\n\n");
    (
        split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut name_split = l.split('{');
                let name = name_split.next().unwrap().to_string();
                let rest = name_split.next().unwrap();
                let condition_caputres = CONDITION_REGEX.captures_iter(rest);
                let mut rules = vec![];
                for captures in condition_caputres {
                    rules.push(Rule {
                        target_feature: Feature::from_char(
                            captures.get(1).unwrap().as_str().chars().next().unwrap(),
                        ),
                        operator: captures.get(2).unwrap().as_str().chars().next().unwrap(),
                        threshold: captures.get(3).unwrap().as_str().parse().unwrap(),
                        target_workflow: captures.get(4).unwrap().as_str().to_string(),
                    })
                }
                let last = rest.split(',').last().unwrap();
                let final_workflow: String = last.chars().take(last.len() - 1).collect();
                (
                    name,
                    Workflow {
                        rules,
                        final_workflow,
                    },
                )
            })
            .collect(),
        split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut vals = l.split(',').map(|s| {
                    s.chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse()
                        .unwrap()
                });
                let mut values = HashMap::new();
                values.insert(Feature::X, vals.next().unwrap());
                values.insert(Feature::M, vals.next().unwrap());
                values.insert(Feature::A, vals.next().unwrap());
                values.insert(Feature::S, vals.next().unwrap());
                Part { values }
            })
            .collect(),
    )
}

fn find_next_workflow(workflow: &Workflow, part: &Part) -> String {
    for rule in &workflow.rules {
        match rule.operator {
            '<' => {
                if part.values.get(&rule.target_feature).unwrap() < &rule.threshold {
                    return rule.target_workflow.clone();
                }
            }
            '>' => {
                if part.values.get(&rule.target_feature).unwrap() > &rule.threshold {
                    return rule.target_workflow.clone();
                }
            }
            _ => panic!(),
        }
    }
    workflow.final_workflow.clone()
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &[Part]) -> usize {
    parts
        .iter()
        .map(|part| {
            let mut current_workflow = "in".to_string();
            while current_workflow != "R" && current_workflow != "A" {
                current_workflow =
                    find_next_workflow(workflows.get(&current_workflow).unwrap(), part);
            }
            if current_workflow == "A" {
                part.values.values().sum::<usize>()
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let (workflows, parts) = parse(include_str!("../input.txt"));
    println!("{}", part1(&workflows, &parts));
}
