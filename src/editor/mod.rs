mod buffer;
mod cursor;
mod commands;

use buffer::Buffer;
use cursor::Cursor;
use crossterm::event::KeyCode;
use crate::config::Settings;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    settings: Settings,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
            settings: Settings::new(),
        }
    }

    pub fn new_with_settings(settings: Settings) -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
            settings,
        }
    }

    pub fn process_keypress(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Tab => self.handle_tab(),
            KeyCode::Esc => return true,
            _ => {}
        }
        false
    }

    fn insert_char(&mut self, c: char) {
        self.buffer.insert((self.cursor.row, self.cursor.col), c);
        self.cursor.move_right(self.buffer.get_line(self.cursor.row).map_or(0, |l| l.len()));
    }

    fn delete_char(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.move_left();
            self.buffer.delete((self.cursor.row, self.cursor.col));
        } else if self.cursor.row > 0 {
            // Join with previous line
            let current_row = self.cursor.row;
            self.move_cursor_up();
            self.cursor.col = self.buffer.get_line(self.cursor.row).map_or(0, |l| l.len());
            self.buffer.join_lines(self.cursor.row);
            self.buffer.delete_line(current_row);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.move_left();
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.buffer.get_line(self.cursor.row).map_or(0, |l| l.len());
        }
    }

    fn move_cursor_right(&mut self) {
        if let Some(line) = self.buffer.get_line(self.cursor.row) {
            if self.cursor.col < line.len() {
                self.cursor.move_right(line.len());
            } else if self.cursor.row < self.buffer.len() - 1 {
                self.cursor.row += 1;
                self.cursor.col = 0;
            }
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.move_up();
            let line_len = self.buffer.get_line(self.cursor.row).map_or(0, |l| l.len());
            if self.cursor.col > line_len {
                self.cursor.col = line_len;
            }
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor.row < self.buffer.len() - 1 {
            self.cursor.move_down(self.buffer.len());
            let line_len = self.buffer.get_line(self.cursor.row).map_or(0, |l| l.len());
            if self.cursor.col > line_len {
                self.cursor.col = line_len;
            }
        }
    }

    fn handle_tab(&mut self) {
        if self.settings.use_spaces {
            for _ in 0..self.settings.tab_size {
                self.insert_char(' ');
            }
        } else {
            self.insert_char('\t');
        }
    }

    pub fn get_content(&self) -> Vec<&str> {
        (0..self.buffer.len())
            .filter_map(|i| self.buffer.get_line(i))
            .collect()
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor.row, self.cursor.col)
    }
}