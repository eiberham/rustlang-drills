//! Bundle abstraction.
//!
//! Provides a bundle in charge of holding blocks for the game to
//! draw on every game cycle.
//!

use crate::{block::*};

pub struct Bundle<Block : Copy> {
  pub values: Vec<Block>
}

impl Bundle<Block> {
  pub fn new() -> Self {
    Self { values:  Vec::new() }
  }

  pub fn push(&mut self, value: Block) -> () {
    self.values.push(value);
  }

  pub fn update(&mut self, value: Block) -> () {
    self.values[self.index_of(value).unwrap()].position.x = value.position.x;
    self.values[self.index_of(value).unwrap()].position.y = value.position.y;
  }

  pub fn index_of(
    &mut self,
    value: Block) -> Option<usize> where Block: PartialEq<Block> {
    let mut index = None;

    for (i, block) in self.values.into_iter().enumerate() {
      if block == value {
        index = Some(i);
      }
    }

    index
  }

  pub fn is_empty(&self) -> bool {
    self.values.is_empty()
  }
}

/* impl<T> Copy for Bundle<T> where T: Copy {}
impl<T> Clone for Bundle<T> where T: Copy {
    fn clone(&self) -> Self {
        *self
    }
} */


// The `Iterator` trait only requires a method to be defined
// for the `next` element.
/* impl <T>Iterator for Bundle<T> {
    // We can refer to this type using Self::Item
    type Item = T;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current: Vec<T> = self.values;

        self.curr = self.next;
        self.next = current + self.next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(current)
    }
} */