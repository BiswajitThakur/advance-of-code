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
}
