#[derive(Debug, Clone)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn from_string(string: &str) -> Self {
        let mut result = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        let split = string.split(',');
        for color in split {
            let mut split = color.trim().split(' ');
            let count: u32 = split.next().unwrap().parse().unwrap();
            match split.next().unwrap().trim() {
                "red" => result.red = count,
                "green" => result.green = count,
                "blue" => result.blue = count,
                _ => panic!(),
            }
        }
        result
    }

    fn is_possible(&self) -> bool {
        if self.red > 12 || self.green > 13 || self.blue > 14 {
            return false;
        }
        true
    }
    fn as_tuple(&self) -> (u32, u32, u32) {
        (self.red, self.green, self.blue)
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let mut split = line.split(':');
        Self {
            id: split
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            draws: split
                .next()
                .unwrap()
                .split(';')
                .map(Draw::from_string)
                .collect(),
        }
    }

    fn is_possible(&self) -> bool {
        self.draws.iter().filter(|draw| draw.is_possible()).count() == self.draws.len()
    }

    fn power(&self) -> u32 {
        let tuple_draws: Vec<_> = self.draws.iter().map(|draw| draw.as_tuple()).collect();
        tuple_draws.iter().map(|(r, _, _)| r).max().unwrap_or(&0)
            * tuple_draws.iter().map(|(_, g, _)| g).max().unwrap_or(&0)
            * tuple_draws.iter().map(|(_, _, b)| b).max().unwrap_or(&0)
    }
}

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games.iter().map(|game| game.power()).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let games: Vec<Game> = input.lines().map(Game::from_line).collect();
    println!("{}", part1(&games));
    println!("{}", part2(&games));
}
