//! Tetromino interface.
//!
//! Provides an interface and some enums to be used
//! by blocks.
//!


use ggez::{
    graphics::{ Canvas },
    Context, GameError
};
use rand::{
    distributions::{ Distribution, Standard },
    Rng
};
use core::fmt::Debug;

/// Trait that defines the tetromino's behaviour
pub trait Tetromino {
  fn rotate(&mut self);
  fn move_l(&mut self);
  fn move_r(&mut self);
  fn draw(
    &mut self,
    canvas: &mut Canvas,
    context: &mut Context) -> Result<(), GameError>;
  fn clone_dyn(&self) -> Box<dyn Tetromino>;
}


impl Clone for Box<dyn Tetromino> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

/// Different kind of shapes for tetrominoes
#[derive(Debug, Copy, Clone, smart_default::SmartDefault, PartialEq )]
pub enum Shape {
  #[default] I, O, T, S, Z, J, L
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
  pub x: f32,
  pub y: f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Orientations that could potentially change
/// the tetromino to
#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

/// In order to get a random shape with rand::random()
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

