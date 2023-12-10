use std::collections::HashMap;

fn card_to_int(card: char, part2: bool) -> u32 {
    if let Some(v) = card.to_digit(10) {
        return v;
    }
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if part2 {
                0
            } else {
                11
            }
        }

        'T' => 10,
        _ => panic!(),
    }
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn from_line(line: &str, part2: bool) -> Self {
        let mut line_iter = line.split(' ');
        let cards: Vec<u32> = line_iter
            .next()
            .unwrap()
            .chars()
            .map(|c| card_to_int(c, part2))
            .collect();
        let bid: u32 = line_iter.next().unwrap().parse().unwrap();
        let mut card_amounts: HashMap<u32, u32> = HashMap::from_iter(
            cards
                .clone()
                .into_iter()
                .map(|card| (card, cards.iter().filter(|c| **c == card).count() as u32)),
        );
        if part2 && card_amounts.contains_key(&0) {
            let j_amount = *card_amounts.get(&0).unwrap();
            let mut amounts_vec: Vec<(u32, u32)> = card_amounts.clone().into_iter().collect();
            amounts_vec.sort_unstable_by(|(_, v1), (_, v2)| v2.cmp(v1));
            let most_frequent = match amounts_vec[0] {
                (0, _) => amounts_vec.get(1).unwrap_or(&(0, 0)).0,
                _ => amounts_vec[0].0,
            };
            if most_frequent != 0 {
                if let Some(x) = card_amounts.get_mut(&most_frequent) {
                    *x += j_amount;
                }
                card_amounts.remove(&0);
            }
        }
        let hand_type = match card_amounts.keys().len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_amounts.values().any(|x| x == &4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_amounts.values().any(|x| x == &3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!(),
        };
        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.hand_type.partial_cmp(&other.hand_type).unwrap();
        if cmp != std::cmp::Ordering::Equal {
            return cmp;
        }
        for i in 0..5 {
            match self.cards[i].cmp(&other.cards[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}

fn get_total_winnings(hands: &[Hand]) -> u32 {
    let mut hands: Vec<_> = hands.iter().collect();
    hands.sort_unstable();
    (0..hands.len())
        .map(|i| hands[i].bid * (i + 1) as u32)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let hands: Vec<Hand> = input.lines().map(|l| Hand::from_line(l, false)).collect();
    println!("{}", get_total_winnings(&hands));
    let hands: Vec<Hand> = input.lines().map(|l| Hand::from_line(l, true)).collect();
    println!("{}", get_total_winnings(&hands));
}
