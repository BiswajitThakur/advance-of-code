fn main() {
    println!(
        "calibration values: {}",
        calibration_value(include_str!("./../../input.txt"))
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MyNum {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<usize> for MyNum {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => unreachable!(),
        }
    }
}

impl From<MyNum> for usize {
    fn from(value: MyNum) -> Self {
        match value {
            MyNum::Zero => 0,
            MyNum::One => 1,
            MyNum::Two => 2,
            MyNum::Three => 3,
            MyNum::Four => 4,
            MyNum::Five => 5,
            MyNum::Six => 6,
            MyNum::Seven => 7,
            MyNum::Eight => 8,
            MyNum::Nine => 9,
        }
    }
}

impl From<&MyNum> for char {
    fn from(value: &MyNum) -> Self {
        match value {
            MyNum::Zero => '0',
            MyNum::One => '1',
            MyNum::Two => '2',
            MyNum::Three => '3',
            MyNum::Four => '4',
            MyNum::Five => '5',
            MyNum::Six => '6',
            MyNum::Seven => '7',
            MyNum::Eight => '8',
            MyNum::Nine => '9',
        }
    }
}

impl From<&MyNum> for &'static str {
    fn from(value: &MyNum) -> Self {
        match value {
            MyNum::Zero => "zero",
            MyNum::One => "one",
            MyNum::Two => "two",
            MyNum::Three => "three",
            MyNum::Four => "four",
            MyNum::Five => "five",
            MyNum::Six => "six",
            MyNum::Seven => "seven",
            MyNum::Eight => "eight",
            MyNum::Nine => "nine",
        }
    }
}

impl MyNum {
    fn find<T: AsRef<str>>(&self, value: T) -> Option<usize> {
        let d: &str = self.into();
        let c: char = self.into();
        let value = value.as_ref();
        let mut index = value.find(c);
        if let Some(v) = value.find(d) {
            if (index.is_some() && v < index.unwrap()) || index.is_none() {
                index = Some(v);
            }
        }
        index
    }
    fn rfind<T: AsRef<str>>(&self, value: T) -> Option<usize> {
        let d: &str = self.into();
        let c: char = self.into();
        let value = value.as_ref();
        let mut index = value.rfind(c);
        if let Some(v) = value.rfind(d) {
            if (index.is_some() && v > index.unwrap()) || index.is_none() {
                index = Some(v);
            }
        }
        index
    }
}

fn find_num<T: AsRef<str>>(value: T) -> isize {
    let value = value.as_ref();
    let mut left_index = None;
    let mut left: isize = 0;
    for i in 0usize..=9 {
        let v: MyNum = i.into();
        if let Some(index) = v.find(value) {
            if (left_index.is_some() && index < left_index.unwrap()) || left_index.is_none() {
                left = i as isize;
                left_index = Some(index);
            }
        }
    }
    let mut right_index = None;
    let mut right: isize = 0;
    for i in 0usize..=9 {
        let v: MyNum = i.into();
        if let Some(index) = v.rfind(value) {
            if (right_index.is_some() && index > right_index.unwrap()) || right_index.is_none() {
                right = i as isize;
                right_index = Some(index);
            }
        }
    }
    (left * 10) + right
}

fn calibration_value<T: AsRef<str>>(value: T) -> isize {
    value.as_ref().lines().map(find_num).sum()
}

#[cfg(test)]
mod tests {
    use crate::{calibration_value, find_num};

    #[test]
    fn test_find_num() {
        let input = "";
        let got = find_num(input);
        assert_eq!(got, 0);
        let input = "a3bninec1def2k9k";
        let got = find_num(input);
        assert_eq!(got, 39);
        let input = "abninec1def2kk";
        let got = find_num(input);
        assert_eq!(got, 92);
        let input = "abfive4ninec1def2kkseven";
        let got = find_num(input);
        assert_eq!(got, 57);
        let input = "a3bfive4ninec1def2kkseven";
        let got = find_num(input);
        assert_eq!(got, 37);
        let input = "a3bfive4ninec1def2kkseveneight2";
        let got = find_num(input);
        assert_eq!(got, 32);
    }

    #[test]
    fn test_calibration_value() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let got = calibration_value(input);
        assert_eq!(got, 142);
    }
    #[test]
    fn test_calibration_value1() {
        let input = "


            ";
        let got = calibration_value(input);
        assert_eq!(got, 0);
    }
}
