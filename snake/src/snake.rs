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
      w: 0x20 as f32,
      h: 0x20 as f32
    };

    let position = Position::new(0x40 as f32, 0x40 as f32);
    let head = Rect::new(position.x, position.y, size.w, size.h);
    let mut body =  LinkedList::new();
    body.push_back(Rect::new(32.0, 64.0, size.w, size.h));

    Snake {
      head,
      previous: Direction::Right,
      current: Some(Direction::Right),
      body
    }
  }

  pub fn draw(&mut self, canvas: &mut Canvas) -> () {

    for square in self.body.iter() {
      canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(*square)
                .color(graphics::Color::GREEN),
        );
    }

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head)
                .color(graphics::Color::YELLOW),
        );
  }

  pub fn update(&mut self) {
    println!("updating the snake's position based on its current direction");

    match &self.current {
      Some(current) => {
        match current {
          Direction::Right => {
            // if !matches!(self.previous, Direction::Right ) {}
            self.body.push_front(self.head);
            self.head = Rect::new(self.head.x + 32.0, self.head.y, self.head.w, self.head.h);
            self.body.pop_back();

          }
          Direction::Left => {
            self.body.push_front(self.head);
            self.head = Rect::new(self.head.x - 32.0, self.head.y, self.head.w, self.head.h);
            self.body.pop_back();
          }
          Direction::Up => {
            self.body.push_front(self.head);
            self.head = Rect::new(self.head.x, self.head.y -32.0, self.head.w, self.head.h);
            self.body.pop_back();
          }
          Direction::Down => {
            self.body.push_front(self.head);
            self.head = Rect::new(self.head.x, self.head.y + 32.0, self.head.w, self.head.h);
            self.body.pop_back();
          }
        }
      }
      None => ()
    }
  }
}