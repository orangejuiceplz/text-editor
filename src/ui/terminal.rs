use std::io::{self, Write};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor,
    event::{self, Event, KeyCode},
    style::Print,
};
use crate::editor::Editor;

pub struct Terminal {
    size: (u16, u16),
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::size()?;
        Ok(Self { size })
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        execute!(io::stdout(), Clear(ClearType::All))?;
        execute!(io::stdout(), cursor::MoveTo(0, 0))
    }

    pub fn read_key(&self) -> Result<KeyCode, std::io::Error> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event.code);
            }
        }
    }

    pub fn refresh(&mut self, editor: &Editor) -> Result<(), std::io::Error> {
        execute!(io::stdout(), cursor::Hide)?;
        execute!(io::stdout(), Clear(ClearType::All))?;

        let content = editor.get_content();
        for (i, line) in content.iter().enumerate() {
            execute!(io::stdout(), cursor::MoveTo(0, i as u16))?;
            execute!(io::stdout(), Print(line))?;
        }

        let (row, col) = editor.cursor_position();
        execute!(io::stdout(), cursor::MoveTo(col as u16, row as u16))?;
        execute!(io::stdout(), cursor::Show)?;
        io::stdout().flush()
    }

    pub fn size(&self) -> (u16, u16) {
        self.size
    }
}