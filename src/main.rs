use std::path::PathBuf;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

use std::fs::OpenOptions;
use std::io::Write;

mod app_logic;
use app_logic::AppLogic;
use app_logic::Events;
mod display_manager;
use display_manager::render;

enum LoopControl {
    Continue,
    Break,
    Normal,
}

fn handle_keyboard(app: &mut AppLogic) -> Result<LoopControl> {
    let Event::Key(key) = event::read()? else {
        return Ok(LoopControl::Continue);
    };
    if let event::KeyCode::Char(c) = key.code {
        if c == 'q' {
            return Ok(LoopControl::Break);
        }
        if c == 'j' {
            app.handle_event(Events::SelectNext);
        }
        if c == 'k' {
            app.handle_event(Events::SelectPrev);
        }
        if c == 's' {
            app.handle_event(Events::ChoseSelection);
        }
    }
    Ok(LoopControl::Normal)
}

fn main() -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("app.log")?;
    let mut app = AppLogic::new();
    let home = std::env::var("HOME")?;
    let wallpaper_path = PathBuf::from(home).join("Pictures").join("Wallpapers");
    app.update_items(wallpaper_path)?;
    app.select(0);
    color_eyre::install()?;
    let terminal = ratatui::init();
    writeln!(file, "Got to the main loop")?;

    let result = run(terminal, &mut app);
    ratatui::restore();
    result
}
fn run(mut terminal: DefaultTerminal, app: &mut AppLogic) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;

        match handle_keyboard(app).unwrap() {
            LoopControl::Continue => continue,
            LoopControl::Break => break,
            LoopControl::Normal => {}
        }
    }
    Ok(())
}
