///
/// The `size`'s w and h will define the tile's dimensions,
/// respectively.
pub struct Size<T> {
  w: T,
  h: T
}

impl<T> Size<T> where
    T: Copy {
  pub fn new(w: T, h: T) -> Self {
    Self { w, h }
  }

  pub fn width(&self) -> T {
    self.w
  }

  pub fn height(&self) -> T {
    self.h
  }
}