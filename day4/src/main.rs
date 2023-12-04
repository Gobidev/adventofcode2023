use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    have_numbers: Vec<u32>,
}

lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new(r"Card *(\d*):(( ?\d* )*)\|(( ?\d* ?)*)").unwrap();
}

fn get_numbers_from_capture(capture_string: &str) -> Vec<u32> {
    capture_string
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|num| num.trim().parse().unwrap())
        .collect()
}

impl Card {
    fn from_line(line: &str) -> Self {
        let captures = CARD_REGEX.captures(line).unwrap();
        Self {
            id: captures.get(1).unwrap().as_str().parse().unwrap(),
            winning_numbers: get_numbers_from_capture(captures.get(2).unwrap().as_str()),
            have_numbers: get_numbers_from_capture(captures.get(4).unwrap().as_str()),
        }
    }
    fn points_worth(&self) -> u32 {
        self.have_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

fn part1(cards: &[Card]) -> u32 {
    let base: u32 = 2;
    cards
        .iter()
        .map(|card| match card.points_worth() {
            0 => 0,
            c => base.pow(c - 1),
        })
        .sum()
}

fn part2(cards: &[Card]) -> u32 {
    let mut amounts: HashMap<u32, u32> = HashMap::from_iter(cards.iter().map(|card| (card.id, 1)));
    for card in cards {
        let card_amount = *amounts.get(&card.id).unwrap();
        for i in card.id + 1..=card.id + card.points_worth() {
            if let Some(amount) = amounts.get_mut(&i) {
                *amount += card_amount;
            }
        }
    }
    amounts.values().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let cards: Vec<Card> = input.lines().map(Card::from_line).collect();
    println!("{}", part1(&cards));
    println!("{}", part2(&cards));
}
