use ggez::{
  graphics::{ self, Color, Canvas, Image },
  Context
};

use crate::tetromino::*;

#[derive(Debug, Copy, Clone)]
pub enum Cell {
  Void,
  Full(Color)
}

impl Cell {
  pub fn is_full(&self) -> bool {
    match *self {
      Cell::Full(_) => true,
      _ => false
    }
  }
}

/// a 12x30 board
#[derive(Debug, Clone, Copy)]
pub struct Board {
  pub cells: [[Cell; 12]; 30]
}

impl Board {
  pub fn new() -> Self {
    Self { cells: [[Cell::Void; 12]; 30] }
  }

  pub fn fill(
    &mut self,
    positions: Vec<Position>,
    color: Color) -> () {
      for Position { x, y} in positions.into_iter() {
        let row: usize = (y as i32 / 32)
          .try_into()
          .unwrap();

        let col: usize = (x as i32 / 32)
          .try_into()
          .unwrap();

        self.cells[row][col] = Cell::Full(color);
      };
  }

  pub fn render(&self, canvas: &mut Canvas, ctx : &mut Context) -> () {
    let image = Image::from_path(ctx, "/block.png").unwrap();
    for i in 0..30 {
      for j in 0..12 {
        match self.cells[i][j] {
          Cell::Full( color ) => {
            canvas.draw(
            &image,
            graphics::DrawParam::new()
              .dest([j as f32 * 32., i as f32 * 32.])
              .color(color)
            );
          }
          Cell::Void => {}
        }
      }
    }
  }

  // clears out the board's row
  pub fn clear(&mut self, row: usize) -> () {
    // TODO:
    // investigate how to check if in a 2 dimensional array all elements in a specific row hold a particular value.
    for i in 0..12 {
      self.cells[row][i] = Cell::Void;
    }
  }
}