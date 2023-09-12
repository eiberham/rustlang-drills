use crate::tetromino::*;

/// a 12x30 board
#[derive(Debug)]
pub struct Board {
  pub cells: [[usize; 12]; 30]
}

impl Board {
  pub fn new() -> Self {
    Self { cells: [[0; 12]; 30] }
  }

  pub fn ocupy(&mut self, positions: Vec<Position>) -> () {
    for Position { x, y} in positions.into_iter() {
      let row: usize = (x as i32 % 32).try_into()
        .unwrap();

      let col: usize = (y as i32 % 32).try_into()
        .unwrap();

      self.cells[row][col] = 1;
    };
  }

  // clears out the board's row
  pub fn clear(&self) -> () {

  }
}