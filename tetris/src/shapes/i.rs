pub struct I;

impl Factory for I {
  pub fn create(&self) -> Box<dyn Self> {
    self.render()
  }
}

impl Tetromino for I {
  /// Rotates the tetromino in clockwise
  /// direction
  pub fn rotate(&self) -> () {

  }

  /// Moves the tetromino to the left
  pub fn move_l(&self) -> () {

  }

  /// Moves the tetromino to the right
  pub fn move_r(&self) -> () {

  }

  pub fn render(
    &self,
    canvas: &mut Canvas,
    context: &mut Context ) {

  }
}