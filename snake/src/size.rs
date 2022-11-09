///
/// The `size`'s w and h will define the tile's dimensions,
/// respectively.
pub struct Size<T> {
  w: T,
  h: T
}

impl<T> Size<T> where
    T: Copy {
  /// Creates a new instance
  pub fn new(w: T, h: T) -> Self {
    Self { w, h }
  }

  /// Returns the width
  pub fn get_width(&self) -> T {
    self.w
  }

  /// Returns the height
  pub fn get_height(&self) -> T {
    self.h
  }
}