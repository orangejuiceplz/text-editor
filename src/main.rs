mod editor;
mod ui;
mod syntax;
mod config;
mod plugins;

use editor::Editor;
use ui::Terminal;

fn main() {
    let mut terminal = Terminal::new().expect("Failed to initialize terminal");
    let mut editor = Editor::new();

    loop {
        terminal.refresh(&editor);
        if let Some(key) = terminal.read_key() {
            if editor.process_keypress(key) {
                break;
            }
        }
    }

    terminal.clear().expect("Can't clear terminal");
}