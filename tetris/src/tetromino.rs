//! Tetromino interface.
//!
//! Provides an interface and some enums to be used
//! by blocks.
//!


use ggez::{
    graphics::{ self, Color, Canvas },
    Context, GameError
};
use rand::{
    distributions::{ Distribution, Standard },
    Rng
};
use core::fmt::Debug;

use crate::squares::*;

/// Trait that defines the tetromino's behaviour
pub trait Tetromino {
  fn rotate(&mut self);
  fn move_l(&mut self);
  fn move_r(&mut self);
  fn draw(
    &mut self,
    canvas: &mut Canvas,
    context: &mut Context) -> Result<(), GameError>;
}

/// Different kind of shapes for tetrominoes
#[derive(Debug, Copy, Clone, smart_default::SmartDefault, PartialEq )]
pub enum Shape {
  #[default] I, O, T, S, Z, J, L
}

impl Shape {
    pub fn color(&self) -> Color {
        match &self {
            Shape::L => Color::BLUE,
            Shape::J => Color::YELLOW,
            Shape::T => Color::GREEN,
            Shape::I => Color::MAGENTA,
            Shape::Z => Color::WHITE,
            Shape::S => Color::RED,
            Shape::O => Color::CYAN
        }
    }

    pub fn matrix(&self) -> [[u8; 4]; 4] {
        match &self {
            Shape::L => squares[0][0],
            Shape::J => squares[1][0],
            Shape::T => squares[2][0],
            Shape::I => squares[3][0],
            Shape::Z => squares[4][0],
            Shape::S => squares[5][0],
            Shape::O => squares[6][0]
        }
    }
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

