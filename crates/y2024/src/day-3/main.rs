use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    part1(input);
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let dates: u128 = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [num1, num2]) = caps.extract();
            num1.parse::<u128>().unwrap() * num2.parse::<u128>().unwrap()
        })
        .sum();
    println!("sum of multiplications: {}", dates);
}
