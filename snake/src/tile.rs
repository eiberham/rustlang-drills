pub struct Tile {
  pub place: Place
}

impl Tile {
  pub fn draw(&mut self) {

  }
}

// thoughts:
// tiles will hold up a place struct
// the snake's head is going to be a tile
// so snake's body will be a linked list of tiles
// tile and food structs will hold a place member
// i'll check if snake.head.place == food.place to determine the snake's collision

// there's an issue tough :
// f32 types does not implement the trait Eq, PartialEq
// figure out how to solve that