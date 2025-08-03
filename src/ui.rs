use ratatui::{layout::{Constraint, Layout}, style::Stylize, text::{Line, Span}, widgets::Paragraph, Frame};
use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::vertical([Constraint::Length(17), Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.area());

    frame.render_widget(&app.board, layout[0]);

    frame.render_widget(Paragraph::new(Line::from(vec![
        Span::raw("^c ").red(),
        Span::raw("Quit  ")
    ])), layout[2]);
}