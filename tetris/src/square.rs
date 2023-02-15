struct Position {
  x: f32,
  y: f32
}

impl Position {
  fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}

pub struct Square {
  position: Position
}

impl Square {
  pub fn new(x: f32, y: f32) -> Self {
    Square {
      position: Position::new(x, y)
    }
  }
}