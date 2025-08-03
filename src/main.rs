mod board;
mod letter;
mod score;
mod lexicon;
mod legal_moves;
mod ui;
mod app;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{backend::Backend, Terminal};
use crate::{app::App, ui::ui};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if let KeyCode::Char('c') = key_event.code && key_event.modifiers.contains(KeyModifiers::CONTROL) { break Ok(()) }
            }
            _ => {}
        };
    }
}