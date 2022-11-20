//! Tile abstraction.
//!
//! Provides an abstraction over a tile. Is the square shape the
//! snake and food are made of.
//!

use rand::{ self };
use rand::seq::SliceRandom;
use ggez::graphics::{ Rect };

/// Trait used to draw anything to the canvas.
/// It will be implemented for the tile which is used by the snake
/// and the food.
pub trait Drawable<T> {
  fn draw(&mut self) -> T;
}

/// Represents the position of a tile within the playground taking
/// into account the cartesian plane.
///
#[derive(PartialEq, Copy, Clone, smart_default::SmartDefault)]
pub struct Tile {
  pub x: f32,
  pub y: f32
}

impl Tile {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  /// Returns a random position for the tile filtering out positions
  /// that have already been filled.
  pub fn get_rand() -> Self {
    let cells: Vec<u32> = (96..960)
      .step_by(32)
      .map(|v| v)
      .collect();

    let randomize = move || *cells.to_vec()
        .choose(&mut rand::thread_rng())
        .unwrap();

    let x = randomize();
    let y = randomize();

    Self {
      x: x as f32,
      y: y as f32
    }
  }

  /// Moves the tile x positions in the x axis.
  pub fn move_x(&mut self, x : f32) -> () {
    self.x += x;
  }

  /// Moves the tile y positions in the y axis.
  pub fn move_y(&mut self, y: f32) -> () {
    self.y += y;
  }
}

/// Implementation of the drawable trait for the tile. The figure
/// returned will be a rectangle.
impl Drawable<Rect> for Tile {
  fn draw(&mut self) -> Rect {
    Rect::new(self.x, self.y, 32.0, 32.0)
  }
}
