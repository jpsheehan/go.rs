use core::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellState
{
    None,
    White,
    Black
}

impl Display for CellState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            CellState::Black => "Black",
            CellState::White => "White",
            _ => "None",
        })
    }
}