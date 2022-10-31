use ggez::graphics::{ self, Rect, Canvas };
use std::collections::{ LinkedList };

pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

pub struct Size<T> {
  w: T,
  h: T
}

pub struct Position<T> {
  x: T,
  y: T
}

impl <T> Position<T> {
  pub fn new(x: T, y: T) -> Self {
    Self {
      x,
      y
    }
  }
}

pub struct Snake {
  pub head: Rect,
  pub previous: Direction,
  pub current: Option<Direction>,
  pub body: LinkedList<Rect>,
}

impl Snake {
  // associated function: struct impl fn that don't take self as parameter
  pub fn new() -> Snake {

    let size = Size {
      w: 32 as f32,
      h: 32 as f32
    };

    let position = Position::new(64 as f32, 64 as f32);
    let head = Rect::new(position.x, position.y, size.w, size.h);

    Snake {
      head,
      previous: Direction::Right,
      current: Some(Direction::Right),
      body: LinkedList::new()
    }
  }

  pub fn draw(&mut self, canvas: &mut Canvas) -> () {
    /* let mut body = LinkedList::new();
    let mut rect = Rect::new(10.0, 10.0, 32.0, 32.0);
    body.push_back(rect);
    self.body = body; */

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head)
                .color(graphics::Color::WHITE),
        );
  }

  pub fn update(&mut self) {
    println!("updating the snake's position based on its current direction");
  }
}