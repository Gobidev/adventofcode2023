#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn is_adjacent(&self, other: &Position) -> bool {
        (self.x as isize - other.x as isize).abs() <= 1
            && (self.y as isize - other.y as isize).abs() <= 1
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    symbol: char,
    position: Position,
}

impl Symbol {
    fn new(symbol: char, position: Position) -> Self {
        Self { symbol, position }
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: String,
    positions: Vec<Position>,
}

impl Number {
    fn new() -> Self {
        Self {
            value: "".to_string(),
            positions: vec![],
        }
    }
    fn value_as_int(&self) -> u32 {
        self.value.parse().expect("Invalid value")
    }
    fn is_adjacent(&self, position: &Position) -> bool {
        self.positions.iter().any(|p| p.is_adjacent(position))
    }
}

fn parse_input(input: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut current_number = Number::new();
        let mut at_number = false;
        for (x, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => {
                    at_number = true;
                    current_number.positions.push(Position::new(x, y));
                    current_number.value.push(char)
                }
                _ => {
                    if at_number {
                        numbers.push(current_number);
                        current_number = Number::new();
                        at_number = false;
                    }
                    if char != '.' {
                        symbols.push(Symbol::new(char, Position::new(x, y)))
                    }
                }
            }
        }
        if at_number {
            numbers.push(current_number);
        }
    }
    (symbols, numbers)
}

fn part1(symbols: &[Symbol], numbers: &[Number]) -> u32 {
    numbers
        .iter()
        .filter(|number| {
            symbols
                .iter()
                .map(|s| &s.position)
                .any(|s| number.is_adjacent(s))
        })
        .map(|num| num.value_as_int())
        .sum()
}

fn part2(symbols: &[Symbol], numbers: &[Number]) -> u32 {
    symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .map(|s| numbers.iter().filter(|n| n.is_adjacent(&s.position)))
        .filter(|adj_num| adj_num.clone().count() == 2)
        .map(|adj_num| adj_num.fold(1, |a, n| a * n.value_as_int()))
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let (symbols, numbers) = parse_input(input);
    println!("{}", part1(&symbols, &numbers));
    println!("{}", part2(&symbols, &numbers));
}
