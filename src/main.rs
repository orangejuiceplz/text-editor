mod editor;
mod ui;
mod syntax;
mod config;
mod plugins;

use std::path::Path;
use editor::Editor;
use ui::Terminal;
use config::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;

    let config_path = Path::new("config.toml");
    let settings = if config_path.exists() {
        Settings::load(config_path)?
    } else {
        let default_settings = Settings::new();
        default_settings.save(config_path)?;
        default_settings
    };

    let mut terminal = Terminal::new()?;
    let mut editor = Editor::new_with_settings(settings);

    loop {
        terminal.refresh(&editor)?;
        if let Ok(key) = terminal.read_key() {
            if editor.process_keypress(key) {
                break;
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    terminal.clear()?;
    
    Ok(())
}