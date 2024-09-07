pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { lines: vec![String::new()] }
    }

    pub fn insert(&mut self, at: (usize, usize), c: char) {
        let (row, col) = at;
        if row >= self.lines.len() {
            self.lines.push(String::new());
        }
        self.lines[row].insert(col, c);
    }

    pub fn delete(&mut self, at: (usize, usize)) -> Option<char> {
        let (row, col) = at;
        if row < self.lines.len() && col < self.lines[row].len() {
            Some(self.lines[row].remove(col))
        } else {
            None
        }
    }

    pub fn join_lines(&mut self, at: usize) {
        if at < self.lines.len() - 1 {
            let next_line = self.lines.remove(at + 1);
            self.lines[at].push_str(&next_line);
        }
    }

    pub fn delete_line(&mut self, at: usize) {
        if at < self.lines.len() {
            self.lines.remove(at);
        }
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
    }

    pub fn get_line(&self, row: usize) -> Option<&str> {
        self.lines.get(row).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
}