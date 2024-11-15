use std::ops::{Range, RangeInclusive};

fn main() {
    let input = include_str!("./input.txt");
    let mut total: u64 = 0;
    for number in IterPartsNumber::from(input) {
        total += number;
    }
    println!("the sum of all of the part {total}");
}

struct IterPartsNumber<'a> {
    whole: &'a str,
    width: usize,
    pos: (usize, usize),
}

impl<'a, T: AsRef<str> + ?Sized> From<&'a T> for IterPartsNumber<'a> {
    #[inline]
    fn from(value: &'a T) -> Self {
        Self {
            whole: value.as_ref(),
            width: value.as_ref().lines().next().unwrap_or_default().len(),
            pos: (0, 0),
        }
    }
}

impl<'a> IterPartsNumber<'a> {
    #[allow(unused)]
    #[inline]
    fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    #[inline]
    fn get(&self) -> Option<char> {
        let z = (self.pos.1 * self.width) + self.pos.1 + self.pos.0;
        self.whole.as_bytes().get(z).map(|v| *v as char)
    }

    #[inline]
    fn index_of(&self, (x, y): (usize, usize)) -> Option<usize> {
        let z = (self.pos.1 * self.width) + self.pos.1 + self.pos.0;
        if z < self.whole.len() {
            Some(z)
        } else {
            None
        }
    }

    #[inline]
    fn get_from(&self, (x, y): (usize, usize)) -> Option<char> {
        if x >= self.width {
            return None;
        }
        self.whole
            .as_bytes()
            .get((y * self.width) + x + y)
            .map(|v| *v as char)
    }

    #[inline]
    fn up(&self) -> Option<char> {
        self.get_from((self.pos.0, self.pos.1.checked_sub(1)?))
    }

    #[inline]
    fn down(&self) -> Option<char> {
        self.get_from((self.pos.0, self.pos.1 + 1))
    }

    #[inline]
    fn left(&self) -> Option<char> {
        self.get_from((self.pos.0.checked_sub(1)?, self.pos.1))
    }

    #[inline]
    fn right(&self) -> Option<char> {
        self.get_from((self.pos.0 + 1, self.pos.1))
    }

    #[inline]
    fn up_left(&self) -> Option<char> {
        let x = self.pos.0.checked_sub(1)?;
        let y = self.pos.1.checked_sub(1)?;
        self.get_from((x, y))
    }

    #[inline]
    fn up_right(&self) -> Option<char> {
        let x = self.pos.0 + 1;
        let y = self.pos.1.checked_sub(1)?;
        if x >= self.width {
            return None;
        }
        self.get_from((x, y))
    }

    #[inline]
    fn down_left(&self) -> Option<char> {
        let x = self.pos.0.checked_sub(1)?;
        let y = self.pos.1 + 1;
        self.get_from((x, y))
    }

    #[inline]
    fn down_right(&self) -> Option<char> {
        let x = self.pos.0 + 1;
        let y = self.pos.1 + 1;
        self.get_from((x, y))
    }

    fn has_symbol(&self) -> bool {
        let f = |v| !matches!(v, '0'..='9' | '.');
        self.up().map(f).unwrap_or(false)
            | self.down().map(f).unwrap_or(false)
            | self.left().map(f).unwrap_or(false)
            | self.right().map(f).unwrap_or(false)
            | self.up_left().map(f).unwrap_or(false)
            | self.up_right().map(f).unwrap_or(false)
            | self.down_left().map(f).unwrap_or(false)
            | self.down_right().map(f).unwrap_or(false)
    }

    #[inline]
    fn set_possition_down_left(&mut self) {
        self.pos.0 = 0;
        self.pos.1 += 1;
    }

    #[inline]
    fn set_possition_next(&mut self) {
        let x = self.pos.0 + 1;
        if x >= self.width {
            self.set_possition_down_left();
        } else {
            self.pos.0 = x;
        }
    }

    fn get_num_range(&self, index: usize) -> Range<usize> {
        let mut left = 0;
        let mut right = 0;
        for i in (0..index).rev() {
            if let Some(c) = self.whole.as_bytes().get(i) {
                if !(*c as char).is_ascii_digit() {
                    left = i;
                    break;
                }
            } else {
                break;
            }
        }
        for i in index..self.whole.len() {
            if let Some(c) = self.whole.as_bytes().get(i) {
                if !(*c as char).is_ascii_digit() {
                    right = i;
                    break;
                }
            } else {
                break;
            }
        }
        left + 1..right
    }

    fn get_adjacent_from(&self, (x, y): (usize, usize)) -> Option<u64> {
        let mut left: u64 = 0;
        let mut right: u64 = 0;
        let mut top: u64 = 0;
        let mut down: u64 = 0;
        let mut top_left: u64 = 0;
        let mut top_right: u64 = 0;
        let mut down_left: u64 = 0;
        let mut down_right: u64 = 0;
        match (x, y) {
            (0, 0) => {
                if let Some(v) = self.index_of((1, 0)) {
                    for c in self.whole[v..].chars() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        left *= 10;
                        left += c as u64 - '0' as u64;
                    }
                };
                if let Some(v) = self.index_of((0, 1)) {
                    //if let Some(u) = self.whole.get(v) {}
                    for c in self.whole[v..].chars() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        left *= 10;
                        left += c as u64 - '0' as u64;
                    }
                };
            }
            _ => {}
        }
        todo!()
    }
}

impl Iterator for IterPartsNumber<'_> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let mut total: u64 = 0;
        let mut flag = false;
        while let Some(v) = self.get() {
            match v {
                '0'..='9' => {
                    total *= 10;
                    total += v as u64 - '0' as u64;
                    if !flag {
                        flag = self.has_symbol();
                    }
                }
                _ => {
                    if flag {
                        self.set_possition_next();
                        return Some(total);
                    } else {
                        total = 0;
                    }
                }
            }
            self.set_possition_next();
        }
        None
    }
}

#[test]
fn test_part_get() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let mut iter = IterPartsNumber::from(input);
    assert_eq!(iter.next(), Some(467));
    assert_eq!(iter.next(), Some(35));
    assert_eq!(iter.next(), Some(633));
    assert_eq!(iter.next(), Some(617));
    assert_eq!(iter.next(), Some(592));
    assert_eq!(iter.next(), Some(755));
    assert_eq!(iter.next(), Some(664));
    assert_eq!(iter.next(), Some(598));
    assert_eq!(iter.next(), None);
    let input = "467..114..
...*......
..35.%695.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let mut v = IterPartsNumber::from(input);

    assert_eq!(v.get_from((6, 3)), Some('#'));
    assert_eq!(v.get_from((9, 9)), Some('.'));
    assert_eq!(v.get_from((7, 9)), Some('8'));
    assert_eq!(v.get_from((0, 0)), Some('4'));

    assert_eq!(v.get_num_range(6), 5..8);
    assert_eq!(v.get_num_range(26), 26..29);

    assert_eq!(v.get(), Some('4'));
    assert_eq!(v.up(), None);
    assert_eq!(v.down(), Some('.'));
    assert_eq!(v.right(), Some('6'));
    assert_eq!(v.up_right(), None);
    assert_eq!(v.up_left(), None);
    assert!(!v.has_symbol());

    v.set_pos((9, 0));
    assert_eq!(v.get(), Some('.'));
    assert_eq!(v.right(), None);
    assert_eq!(v.up(), None);
    assert_eq!(v.up_right(), None);
    assert_eq!(v.up_left(), None);
    assert_eq!(v.down_left(), Some('.'));
    assert!(!v.has_symbol());

    v.set_pos((6, 3));
    assert_eq!(v.get(), Some('#'));
    assert_eq!(v.up(), Some('6'));
    assert_eq!(v.down(), Some('.'));
    assert_eq!(v.up_right(), Some('9'));
    assert_eq!(v.up_left(), Some('%'));
    assert_eq!(v.down_left(), Some('.'));
    assert!(v.has_symbol());

    v.set_pos((0, 1));
    assert_eq!(v.get(), Some('.'));
    assert_eq!(v.up(), Some('4'));
    assert_eq!(v.up_right(), Some('6'));
    assert_eq!(v.up_left(), None);
    assert!(!v.has_symbol());

    v.set_pos((9, 1));
    assert_eq!(v.get(), Some('.'));
    assert_eq!(v.up(), Some('.'));
    assert_eq!(v.up_right(), None);
    assert_eq!(v.up_left(), Some('.'));
}
