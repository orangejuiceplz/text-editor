mod editor;
mod ui;
mod syntax;
mod config;
mod plugins;

use std::path::Path;
use anyhow::{Context, Result};
use editor::Editor;
use ui::Terminal;
use config::Settings;

fn main() -> Result<()> {
    crossterm::terminal::enable_raw_mode().context("Failed to enable raw mode")?;

    let config_path = Path::new("config.toml");
    let settings = if config_path.exists() {
        Settings::load(config_path).context("Failed to load settings")?
    } else {
        let default_settings = Settings::new();
        default_settings.save(config_path).context("Failed to save default settings")?;
        default_settings
    };

    let mut terminal = Terminal::new().context("Failed to initialize terminal")?;
    let mut editor = Editor::new_with_settings(settings);

    loop {
        terminal.refresh(&editor).context("Failed to refresh terminal")?;
        
        match terminal.read_key() {
            Ok(key) => {
                if editor.process_keypress(key) {
                    break; 
                }
            }
            Err(err) => eprintln!("Error reading key: {:?}", err), 
        }
    }

    crossterm::terminal::disable_raw_mode().context("Failed to disable raw mode")?;
    terminal.clear().context("Failed to clear terminal")?;

    Ok(())
}