use ggez::{
    graphics::{ Text, Color, DrawParam, Canvas, FontData },
};
use rand::{
    distributions::{ Distribution, Standard },
    Rng,
};

/// Trait that defines the tetromino's behaviour
pub trait Tetromino {
  fn rotate(&self);
  fn move_l(&self);
  fn move_r(&self);
  fn render(&self, canvas: &mut Canvas, context: &mut Context);
}

/// Different kind of shapes
#[derive(Debug, Copy, Clone, smart_default::SmartDefault )]
pub enum Shape {
  #[default]
  I,
  O,
  T,
  S,
  Z,
  J,
  L
}

/// In order to get random shape with rand::random() ...
/// Refer to https://stackoverflow.com/a/48491021
impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Spinner {
        match rng.gen_range(0..=6) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            6 => Shape::L,
            _ => Shape::I
        }
    }
}

