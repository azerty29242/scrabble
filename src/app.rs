
use crate::board::Board;

#[derive(Debug)]
pub struct App {
    pub board: Board,
}

impl App {
    pub fn new() -> Self {
        Self { board: Board::new() }
    }
}