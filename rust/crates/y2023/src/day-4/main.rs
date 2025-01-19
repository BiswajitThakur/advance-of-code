use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let total = total_paints(input);
    println!("Tatal points: {}", total.0);
}

fn total_paints(value: &str) -> (u64, u64) {
    let mut total: u64 = 0;
    let mut scratchcards = HashMap::new();
    for line in value.lines() {
        let v = winning_scratchcards_from_line(line);
        if let Some(v) = (v.0.len() as u32).checked_sub(1) {
            total += 2u64.pow(v);
        }
        for i in v.1 {
            if let Some(v) = scratchcards.get_mut(&i) {
                *v += 1;
            } else {
                scratchcards.insert(i, 1);
            }
        }
    }
    dbg!(&scratchcards);
    (total, scratchcards.iter().map(|(_, v)| *v as u64).sum())
}

fn winning_scratchcards_from_line(value: &str) -> (Vec<i32>, Vec<usize>) {
    value
        .split_once(':')
        .and_then(|(left, right)| {
            Some((
                left.split_whitespace()
                    .rev()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                right,
            ))
        })
        .and_then(|(card_number, v)| Some((card_number, v.split_once('|').unwrap())))
        .and_then(|(card_number, (left, right))| {
            let left_nums: HashSet<i32> = left
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            let right_nums: HashSet<i32> = right
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            let intersect: Vec<i32> = left_nums.intersection(&right_nums).map(|v| *v).collect();
            let mut cards = Vec::new();
            cards.push(card_number);
            for i in card_number + 1..=card_number + intersect.len() {
                cards.push(i);
            }
            Some((intersect, cards))
        })
        .unwrap_or_default()
}

#[test]
fn test_winning_scratchcards_from_line() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let got = winning_scratchcards_from_line(line);
    assert_eq!(
        got.0.into_iter().collect::<HashSet<i32>>(),
        HashSet::from([48, 83, 17, 86])
    );
    assert_eq!(
        got.1.into_iter().collect::<HashSet<usize>>(),
        HashSet::from([1, 2, 3, 4, 5])
    );
    let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
    let got = winning_scratchcards_from_line(line);
    assert_eq!(
        got.0.into_iter().collect::<HashSet<i32>>(),
        HashSet::from([32, 61])
    );
    assert_eq!(
        got.1.into_iter().collect::<HashSet<usize>>(),
        HashSet::from([2, 3, 4])
    );
    let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let got = winning_scratchcards_from_line(line);
    assert_eq!(got.0.into_iter().collect::<HashSet<i32>>(), HashSet::new());
    assert_eq!(
        got.1.into_iter().collect::<HashSet<usize>>(),
        HashSet::from([6])
    );
}

#[test]
fn test_total_points() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let total = total_paints(input);
    assert_eq!(total, (13, 30));
}
