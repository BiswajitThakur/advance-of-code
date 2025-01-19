use std::ops::Index;

fn main() {
    let input = include_str!("./input.txt");
}

struct Cursor<'a> {
    input: &'a [u8],
    width: usize,
    pos: usize,
}

impl<'a> From<&'a [u8]> for Cursor<'a> {
    fn from(value: &'a [u8]) -> Self {
        let width = unsafe { std::str::from_utf8_unchecked(value) }
            .lines()
            .next()
            .unwrap_or_default()
            .len();
        Self {
            input: value,
            width,
            pos: 0,
        }
    }
}

impl Index<usize> for Cursor<'_> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        self.input.get(index).unwrap()
    }
}

impl<'a> Cursor<'a> {
    fn get(&self, index: usize) -> Option<u8> {
        self.input.get(index).copied()
    }
    fn get_char(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }
    fn set_pos((x, y): (usize, usize)) {
        todo!()
    }
    fn get_line_no(&self) -> usize {
        self.pos.checked_sub(2 * self.width).unwrap_or_default()
    }
    fn get_down_from(&self, pos: usize) -> Option<u8> {
        let index = pos + self.width + 1;
        if index < self.input.len() {
            return self.input.get(index).copied();
        }
        None
    }
    fn get_left_from(&self, pos: usize) -> Option<u8> {
        if let Some(v) = pos.checked_sub(1) {
            return self.input.get(v).copied();
        }
        None
    }
    fn move_right(&mut self) {
        todo!()
    }
    fn move_left_down(&mut self) {
        todo!()
    }
    fn move_rigth_down(&mut self) {
        todo!()
    }
    fn has_left(&self, value: &[u8]) -> bool {
        if value.len() == 0 {
            return true;
        }
        let end = if let Some(v) = self.pos.checked_sub(value.len()) {
            v
        } else {
            return false;
        };
        let mut j = 0;
        for i in self.pos..end {
            if value[j] != self[i] {
                return false;
            }
            j += 1;
        }
        true
    }
}
