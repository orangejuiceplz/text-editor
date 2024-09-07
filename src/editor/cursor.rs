pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn move_right(&mut self, line_len: usize) {
        if self.col < line_len {
            self.col += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
        }
    }

    pub fn move_down(&mut self, buffer_len: usize) {
        if self.row < buffer_len - 1 {
            self.row += 1;
        }
    }
}