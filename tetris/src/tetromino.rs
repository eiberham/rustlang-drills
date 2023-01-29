use ggez::{
    graphics::{ Text, Color, DrawParam, Canvas, FontData },
};
use rand::{
    distributions::{ Distribution, Standard },
    Rng,
};

pub trait Behaviour {
  pub fn rotate();
  pub fn move_l();
  pub fn move_r();
}

#[derive(Debug)]
pub enum Shape {
  I,
  O,
  T,
  S,
  Z,
  J,
  L
}

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

#[derive(smart_default::SmartDefault)]
pub struct Tetromino {
   #[default(rand::random())]
  pub kind: Shape
}

impl Tetromino {
  pub fn new () -> Self {
    Self::default()
  }

  pub fn draw(
    &mut self,
    canvas: &mut Canvas,
    ctx: &mut Context) {

  }
}

impl Behaviour for Tetromino {
  /// Rotates the tetromino in clockwise direction
  pub fn rotate() -> () {

  }

  /// Moves the tetromino to the left
  pub fn move_l() -> () {

  }

  /// Moves the tetromino to the right
  pub fn move_r() -> () {

  }
}

