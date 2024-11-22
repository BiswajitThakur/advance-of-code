use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let total = total_paints(input);
    println!("Tatal points: {}", total);
}

fn total_paints(value: &str) -> u64 {
    let mut total: u64 = 0;
    for line in value.lines() {
        let v = winning_numbers_from_line(line).len() as u32;
        if let Some(v) = v.checked_sub(1) {
            total += 2u64.pow(v);
        }
    }
    total
}

fn winning_numbers_from_line(value: &str) -> Vec<i32> {
    value
        .split_once(':')
        .and_then(|(_, right)| Some(right))
        .and_then(|v| v.split_once('|'))
        .and_then(|(left, right)| {
            let left_nums: HashSet<i32> = left
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            let right_nums: HashSet<i32> = right
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            let intersect: Vec<i32> = left_nums.intersection(&right_nums).map(|v| *v).collect();
            Some(intersect)
        })
        .unwrap_or_default()
}
