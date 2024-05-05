//! Bloard element.
//!
//! In charge of rendering the already landed blocks
//! and clearing out the filled rows.
//!

use std::default::Default;

use ggez::{
  graphics::{ self, Color, Canvas, Image },
  audio::{SoundSource, Source},
  Context
};

extern crate arrayvec;
use arrayvec::ArrayVec;

use crate::utils::Position;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl Default for Cell {
  fn default() -> Self {
    Cell::Void
  }
}

// thirty rows, twelve cols
#[derive(Debug, Clone, Copy)]
pub struct Board {
  pub cells: [[Cell; 12]; 30]
}

impl Board {
  pub fn new() -> Self {
    Self { cells: [[Cell::Void; 12]; 30] }
  }

  // ocupies positions
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

  // renders the board
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

  // clears out the filled rows
  pub fn clear(&mut self, ctx: &mut Context) -> u16 {
    let mut cells = self.cells.to_vec();
    let mut count = 0;

    for i in (0..30).rev() {
      let full: bool = cells[i as usize]
        .iter()
        .all(Cell::is_full);

      if full {
        let mut sound = Source::new(ctx, "/sound.mp3").unwrap();
        sound.play_detached(ctx).unwrap();
        cells.remove(i as usize);
        let row: [Cell; 12] = Default::default();
        cells.insert(0, row);
        count += 1;
      }
    }

    let board: ArrayVec<_, 30> = cells
      .into_iter()
      .collect();

    let board: [[Cell; 12]; 30] = board
      .into_inner()
      .unwrap();

    self.cells = board;
    count
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ggez::graphics::Color;
    use crate::utils::Position;

    #[test]
    fn test_fill() {
        let cells: [[Cell; 12]; 30] = [[Cell::Void; 12]; 30];
        let mut board = Board { cells };
        
        let mut filled: [[Cell; 12]; 30] = [[Cell::Void; 12]; 30];

        // iterate over each cell in first row and fill it
        for j in 0..12 {
            filled[1][j] = Cell::Full(Color::BLUE);
        }
        
        let mut positions: Vec<Position> = Vec::new();
        for i in 0..12 {
          positions.push(Position::new(i as f32 * 32., 32.))
        }
        
        board.fill(positions, Color::BLUE);
        
        assert_eq!(board.cells, filled);
    }
}