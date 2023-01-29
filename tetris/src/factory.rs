use crate::{ tetromino };

pub struct Factory {}

impl Factory {
  pub fn create() -> Tetromino {
    Tetromino::default()
  }
}