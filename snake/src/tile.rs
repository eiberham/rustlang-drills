use rand::{ self };
use rand::seq::SliceRandom;

use ggez::graphics::{ Rect };

#[derive(PartialEq, Copy, Clone)]
pub struct Tile {
  pub x: f32,
  pub y: f32
}

impl Tile {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  pub fn rand() -> Self {
    let cells: Vec<u32> = (0..960)
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

  pub fn move_x(&mut self, x : f32) -> () {
    self.x += x;
  }

  pub fn move_y(&mut self, y: f32) -> () {
    self.y += y
  }

  pub fn draw(&mut self) -> Rect {
    Rect::new(self.x, self.y, 32.0, 32.0)
  }
}
