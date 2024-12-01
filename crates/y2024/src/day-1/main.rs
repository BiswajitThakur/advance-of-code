use std::{collections::HashMap, isize};

fn main() {
    let input = include_str!("./input.txt");
    let nums = parse_input(input);
    let result = total_distance(&mut nums.iter());
    println!("total distance: {}", result);
    let score = similarity_score(nums);
    println!("similarity score: {}", score);
}

fn parse_input<T: AsRef<str>>(value: T) -> Vec<(isize, isize)> {
    value
        .as_ref()
        .lines()
        .map(|v| v.split_whitespace())
        .map(|mut v| (v.next().unwrap(), v.next().unwrap()))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn total_distance<'a, I: Iterator<Item = &'a (isize, isize)>>(iter: I) -> isize {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for (u, v) in iter {
        a.push(u);
        b.push(v);
    }
    a.sort();
    b.sort();
    let mut iter_a = a.into_iter();
    let mut iter_b = b.into_iter();
    let mut total = 0;
    while let (Some(m), Some(n)) = (iter_a.next(), iter_b.next()) {
        total += if m > n { m - n } else { n - m };
    }
    total
}
fn similarity_score<I: IntoIterator<Item = (isize, isize)>>(iter: I) -> isize {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for (u, v) in iter {
        a.push(u);
        b.push(v);
    }
    let mut foo: HashMap<isize, isize> = HashMap::with_capacity(a.len());
    for i in a {
        foo.insert(i, 0);
    }
    for i in b {
        if let Some(v) = foo.get(&i) {
            foo.insert(i, v + 1);
        }
    }
    let mut total = 0;
    for k in foo.keys() {
        total += k * foo.get(k).unwrap();
    }
    total
}
