//! Factory interface.
//!
//! Provides methods to summon new shaped blocks.
//!

use crate::utils::{Shape, Position, Orientation, Direction};
use crate::block::Block;

pub trait Factory {
  /// The factory method. It must be overridden with a concrete
  /// implementation.
  fn create(shape: Shape) -> Block;
}

pub struct Piece;

impl Factory for Piece {
  fn create(shape: Shape) -> Block {
    Block::new(
      shape,
      shape.matrix(),
      Direction::D,
      Orientation::Down,
      shape.color(),
      Position::new(128., 0.)
    )
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factory::Factory;
    use ggez::graphics::Color;
    
    #[test]
    fn test_create_piece() {
        let shape: Shape = Shape::J;
        let block: Block = Piece::create(shape);
        assert_eq!(block.color, Color::YELLOW);
        assert_eq!(block.orientation, Orientation::Down);
        assert_eq!(block.direction, Direction::D);
        assert_eq!(block.position.x, 128.);
        assert_eq!(block.position.y, 0.);
    }
  }