use std::{num, ops};

fn main() {
    let input = include_str!("./input.txt");
    let input = parse_input(input);
    let mut total = 0;
    for line in input {
        if line.is_safe() {
            total += 1;
        }
    }
    println!("Total safe: {}", total);
}

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<isize>> {
    let mut v = Vec::new();
    for line in input.as_ref().lines() {
        let mut u = Vec::new();
        for num in line.split_whitespace() {
            let n = num.parse().unwrap();
            u.push(n);
        }
        v.push(u);
    }
    v
}

trait IsSafe {
    fn is_safe(&self) -> bool;
    fn tolerate_level(&self) -> (bool, Option<isize>);
}

impl IsSafe for Vec<isize> {
    fn is_safe(&self) -> bool {
        let mut iter = self.iter();
        let first = iter.next();
        if first.is_none() {
            return true;
        }
        let first = first.unwrap();
        let second = iter.next();
        if second.is_none() {
            return true;
        }
        let second = second.unwrap();
        let is_increasing = first < second;
        let diff = if is_increasing {
            second - first
        } else {
            first - second
        };
        if diff > 3 || diff == 0 {
            return false;
        }
        let mut prev = second;
        for num in iter {
            if is_increasing {
                if num <= prev {
                    return false;
                }
                let diff = num - prev;
                if diff == 0 || diff > 3 {
                    return false;
                }
            } else {
                if num >= prev {
                    return false;
                }
                let diff = prev - num;
                if diff == 0 || diff > 3 {
                    return false;
                }
            }
            prev = num;
        }
        true
    }
    fn tolerate_level(&self) -> (bool, Option<isize>) {
        if self.len() < 2 {
            return (true, None);
        }
        if self.len() == 3 {
            let inc = self[0] < self[1];
            let diff = if inc {
                self[1] - self[0]
            } else {
                self[0] - self[1]
            };
            if diff == 0 || diff > 3 {
                let inc = self[0] < self[2];
                let diff = if inc {
                    self[2] - self[0]
                } else {
                    self[0] - self[2]
                };
                if diff == 0 || diff > 3 {
                    return (false, None);
                } else {
                    return (true, Some(self[1]));
                }
            } else {
            }
        }
        let mut iter = self.iter().enumerate().peekable();

        /*
        let mut iter = self.iter().peekable();
        let first = iter.next();
        if first.is_none() {
            return (true, None);
        }
        let first = first.unwrap();
        let second = iter.next();
        if second.is_none() {
            return (true, None);
        }
        let second = second.unwrap();
        let is_increasing = first < second;
        let diff = if is_increasing {
            second - first
        } else {
            first - second
        };
        if diff > 3 || diff == 0 {
            return (false, None);
        }
        let mut prev = second;
        let mut to_remove: Option<isize> = None;
        while let Some(current) = iter.next() {
            let nums = (*prev, *current, iter.peek());
            match nums {
                (prev, current, None) => {
                    let diff = if is_increasing {
                        current - prev
                    } else {
                        prev - current
                    };
                    if diff == 0 || diff > 3 {
                        if to_remove.is_none() {
                            return (true, Some(current));
                        } else {
                            return (false, None);
                        }
                    }
                }
                (prev, current, Some(next)) => {
                    let next = **next;
                    let diff = if is_increasing {
                        current - prev
                    } else {
                        prev - current
                    };
                    if diff == 0 || diff > 3 {
                        let is_inc = next > prev;
                        if (is_increasing ^ is_inc) || to_remove.is_some() {
                            return (false, None);
                        }

                    }
                    todo!()
                }
            }
        }*/
        todo!()
    }
}

fn foo(n1: isize, n2: isize, n3: isize, is_increasing: bool) -> (bool, Option<isize>) {
    let inc1 = n1 < n2;
    let inc2 = n2 < n3;
    if inc1 ^ inc2 {
        let diff1 = if inc1 { n2 - n1 } else { n1 - n2 };
        if diff1 == 0 {
            todo!()
        }
    }
    todo!()
}
