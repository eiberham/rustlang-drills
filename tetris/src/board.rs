#[derive(Clone, Copy, Debug, smart_default::SmartDefault,)]
pub struct Cell {
  #[default = false]
  pub value: bool
}

impl Cell {
  pub fn ocupied(&self) -> bool {
    self.value == true
  }
}

/// a 30x30 board
#[derive(Debug)]
pub struct Board {
  pub cells: [[Cell; 30]; 30]
}

impl Board {
  pub fn new() -> Self {
    Self {
      cells: [[Cell::default(); 30]; 30]
    }
  }

  // clears out the board's row
  pub fn clear(&self) -> () {

  }
}