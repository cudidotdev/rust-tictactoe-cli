pub struct Tabs<T> {
    index: isize,
    positions: Vec<(u16, u16, T)>,
}

impl<T> Tabs<T> {
    pub fn new(positions: Vec<(u16, u16, T)>) -> Self {
        Tabs {
            positions,
            index: 0,
        }
    }

    pub fn position(&self) -> (u16, u16) {
        (
            self.positions[self.index as usize].0,
            self.positions[self.index as usize].1,
        )
    }

    pub fn value(&self) -> &T {
        &self.positions[self.index as usize].2
    }

    pub fn next(&mut self) {
        let len = self.positions.len() as isize;

        self.index = (self.index + 1) % len
    }

    pub fn prev(&mut self) {
        let len = self.positions.len() as isize;

        self.index = (self.index - 1 + len) % len
    }
}
