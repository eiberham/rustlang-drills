//! Tile abstraction.
//!
//! Provides an abstraction over a tile. Is the square shape the
//! snake and food are made of.
//!

use rand::{ self };
use rand::seq::SliceRandom;

/// Represents the position of a tile within the playground taking
/// into account the cartesian plane.
///
#[derive(PartialEq, Copy, Clone, smart_default::SmartDefault, Debug)]
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
