pub struct Board {
  pub board_size: (i32, i32)
  pub cell_size: (i32, i32)
}

impl Board {
  pub fn new(board_size: (i32, i32), cell_size: (i32, i32)) -> Board {
    Board {
      board_size,
      cell_size
    }
  }
}