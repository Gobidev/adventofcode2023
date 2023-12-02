#[derive(Debug, Clone)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn new(r: u32, g: u32, b: u32) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
    fn from_string(string: &str) -> Self {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;
        let split = string.split(',');
        for color in split {
            let mut split = color.trim().split(' ');
            let count: u32 = split.next().unwrap().parse().unwrap();
            match split.next().unwrap().trim() {
                "red" => r = count,
                "green" => g = count,
                "blue" => b = count,
                _ => panic!(),
            }
        }
        Self::new(r, g, b)
    }

    fn is_possible(&self) -> bool {
        if self.red > 12 || self.green > 13 || self.blue > 14 {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: u32, draws: Vec<Draw>) -> Self {
        Self { id, draws }
    }

    fn from_line(line: &str) -> Self {
        let mut split1 = line.split(':');
        let id: u32 = split1
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let split2 = split1.next().unwrap().split(';');
        let draws: Vec<Draw> = split2.map(Draw::from_string).collect();
        Self::new(id, draws)
    }

    fn is_possible(&self) -> bool {
        self.draws.iter().filter(|draw| draw.is_possible()).count() == self.draws.len()
    }

    fn power(&self) -> u32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in &self.draws {
            if draw.red > min_red {
                min_red = draw.red;
            }
            if draw.green > min_green {
                min_green = draw.green;
            }
            if draw.blue > min_blue {
                min_blue = draw.blue;
            }
        }
        min_red * min_green * min_blue
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
