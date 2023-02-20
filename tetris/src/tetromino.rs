use ggez::{
    graphics::{ Text, Color, DrawParam, Canvas, FontData },
    Context, GameError
};
use rand::{
    distributions::{ Distribution, Standard },
    Rng
};

use core::fmt::Debug;

/// Trait that defines the tetromino's behaviour
pub trait Tetromino {
  fn rotate(&self);
  fn move_l(&self);
  fn move_r(&self);
  fn draw(&mut self, canvas: &mut Canvas, context: &mut Context) -> Result<(), GameError>;
  fn clone_dyn(&self) -> Box<dyn Tetromino>;
}

impl Clone for Box<dyn Tetromino> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
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

/// Represents the direction the snake could potentially move to
#[derive(Copy, Clone, smart_default::SmartDefault )]
pub enum Direction {
  /// Right
  R,
  /// Left
  L,
  /// Down
  #[default]
  D
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

/// In order to get random shape with rand::random() ...
/// Refer to https://stackoverflow.com/a/48491021
impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
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

