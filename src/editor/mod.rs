mod buffer;
mod cursor;
mod commands;

use buffer::Buffer;
use cursor::Cursor;
use crate::ui::Key;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn process_keypress(&mut self, key: Key) -> bool {
        match key {
            Key::Char(c) => self.insert_char(c),
            Key::Left => self.move_cursor_left(),
            Key::Right => self.move_cursor_right(),
            Key::Up => self.move_cursor_up(),
            Key::Down => self.move_cursor_down(),
            Key::Ctrl('q') => return true,
            _ => {}
        }
        false
    }

    // todo: add insert_char && move cursor && others
}