fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let iter = line.chars().filter(|c| c.is_ascii_digit());
            iter.clone().next().unwrap().to_digit(10).unwrap() * 10
                + iter.last().unwrap().to_digit(10).unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("nineight", "98")
                .replace("eighthree", "83")
                .replace("eightwo", "82")
                .replace("twone", "21")
                .replace("oneight", "18")
                .replace("threeight", "38")
                .replace("fiveight", "58")
                .replace("sevenine", "79")
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");
            let iter = line.chars().filter(|c| c.is_ascii_digit());
            iter.clone().next().unwrap().to_digit(10).unwrap() * 10
                + iter.last().unwrap().to_digit(10).unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
