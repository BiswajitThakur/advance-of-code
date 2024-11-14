use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

fn main() {
    let input = include_str!("./input.txt");
    let target = Game::new(0, 12, 13, 14);
    let mut id_sum = 0;
    let mut power = 0;
    for line in input.lines() {
        let g = Game::from(line);
        power += g.pow();
        if g <= target {
            id_sum += g.id;
        }
    }
    println!("sum of the IDs: {}", id_sum);
    println!("sum of the powers: {}", power);
}

#[derive(Debug, Default, Clone)]
struct Colors {
    r: u64,
    g: u64,
    b: u64,
}

impl From<&str> for Colors {
    fn from(value: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for i in value.split(';') {
            for j in i.split(',') {
                let mut iter = j.split_whitespace();
                match (iter.next().unwrap(), iter.next().unwrap()) {
                    (v, "red") => {
                        let v = v.parse().unwrap();
                        if v > r {
                            r = v;
                        }
                    }
                    (v, "green") => {
                        let v = v.parse().unwrap();
                        if v > g {
                            g = v;
                        }
                    }
                    (v, "blue") => {
                        let v = v.parse().unwrap();
                        if v > b {
                            b = v;
                        }
                    }
                    _ => {}
                }
            }
        }
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Default)]
struct Game {
    id: u64,
    red: u64,
    green: u64,
    blue: u64,
}

impl Game {
    fn new(id: u64, red: u64, green: u64, blue: u64) -> Self {
        Game {
            id,
            red,
            green,
            blue,
        }
    }
    #[inline]
    fn pow(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl Add for Game {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            id: self.id + rhs.id,
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for Game {
    fn add_assign(&mut self, rhs: Self) {
        self.id += rhs.id;
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl PartialEq for Game {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl PartialOrd for Game {
    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        self.red < other.red && self.green < other.green && self.blue < other.blue
    }
    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        self.red > other.red && self.green > other.green && self.blue > other.blue
    }
    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (
            self.red.partial_cmp(&other.red)?,
            self.green.partial_cmp(&other.green)?,
            self.blue.partial_cmp(&other.blue)?,
        ) {
            (Ordering::Equal, Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            (
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
            ) => Some(Ordering::Less),
            (
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
            ) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(':').unwrap();
        let id = left
            .split_whitespace()
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        let body = Colors::from(right);
        Self {
            id,
            red: body.r,
            green: body.g,
            blue: body.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Game;
    #[test]
    fn test_from_str_game() {
        let input = "Game 40: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let got = Game::from(input);
        let want = Game::new(40, 14, 3, 15);
        assert_eq!(got, want);
        let g1 = Game::new(40, 3, 3, 3);
        let g2 = Game::new(35, 3, 3, 3);
        assert!(g1 == g2);
        assert!(g1 <= g2);
        assert!(g1 >= g2);
        let g1 = Game::new(40, 3, 4, 5);
        let g2 = Game::new(35, 39, 33, 48);
        assert!(g1 < g2);
        assert!(g2 > g1);
        assert_eq!(g1.pow(), 3 * 4 * 5);
    }
}
