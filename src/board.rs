use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::{Color, Style}, widgets::{Block, BorderType, Cell, Row, Table, Widget}};

use crate::letter::{FromChar, Letter, ToChar};
use crate::score::PREMIUM_SQUARES;

#[derive(Debug)]
pub struct Board {
    pub primary: [[Letter; 15]; 15],
    pub secondary: [[Letter; 15]; 15],
    pub across: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            primary: [[0; 15]; 15],
            secondary: [[0; 15]; 15],
            across: true,
        }
    }

    pub fn play(&mut self, word: &str, row_index: usize, column_index: usize, across: bool) {
        if across {
            for (index, letter) in word.chars().enumerate() {
                self.primary[row_index][column_index + index] = Letter::from_char(letter);
                self.secondary[column_index + index][row_index] = Letter::from_char(letter);
            }
        } else {
            for (index, letter) in word.chars().enumerate() {
                self.primary[row_index + index][column_index] = Letter::from_char(letter);
                self.secondary[column_index][row_index + index] = Letter::from_char(letter);
            }
        }
    }

    pub fn rotate(&mut self) {
        (self.primary, self.secondary) = (self.secondary, self.primary);
        self.across = !self.across;
    }
}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows: Vec<Row> = self.primary.iter().enumerate().map(|(row_index, row)| {
            Row::new(row.iter().enumerate().map(|(col_index, &cell)| {
                match cell {
                    0 => {
                        let color = match PREMIUM_SQUARES[row_index][col_index] {
                            0 => Color::Green,
                            1 => Color::Cyan,
                            2 => Color::Blue,
                            3 => Color::Magenta,
                            4 => Color::Red,
                            _ => panic!()
                        };
                        Cell::from(" • ").style(Style::default().bg(color))
                    },
                    1..=26 => {
                        let character = (cell as Letter).to_char();
                        Cell::from(format!(" {character} ")).style(Style::default().bg(Color::Yellow))
                    },
                    _ => panic!()
                }
            }).collect::<Vec<_>>())
        }).collect();

        Table::new(rows, &vec![Constraint::Length(3); 15])
            .block(Block::bordered().border_type(BorderType::Rounded).title(" Game Board "))
            .column_spacing(0)
            .render(area, buf);
    }
}

