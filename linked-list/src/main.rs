
//! Linked List
//! This is a simple data structure particularly difficult to implement using rust.
//!
//! Keep always in mind:
//! Each value in rust has a variable that’s called its owner.
//! There can only be one owner at a time.
//! When the owner goes out of scope, the value will be dropped.

use std::fmt::{ self, Display };
use std::rc::Rc;
use std::cell::RefCell;
use core::fmt::{ Debug };

// A list is either empty or has an element followed by another list

#[derive(Debug)]
enum List<T> {
    None,
    Child {
        value: T,
        next: Option<Rc<RefCell<List<T>>>>
    }
}

impl<T: Display> fmt::Display for List<T> where T: Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Child { next: _, value } => {
                println!("value in list is {:?}", value);
                Ok(())
            },
            Self::None => Ok(())
        }
    }
}

impl<T> List<T> where
    T: Debug + Display + Copy {
    /// Returns an empty instance of the list
    pub fn new() -> Self {
        Self::None
    }

    /// Adds a new child to the linked list.
    /// Takes element of any type and then inserts it as the last child node
    pub fn push(&mut self, element: T) -> () {
        match self {
            Self::Child { value: _, next } => { // Use case when it has one or more child
                // if next is different that none
                match next {
                    Some(item) => {
                        let mut_s = &mut *item.borrow_mut();
                        mut_s.push(element);
                    },
                    None => {
                        // Add to the next field a new child
                        let link = Self::Child {
                            value: element,
                            next: Some(
                                Rc::new(
                                    RefCell::new(List::new())
                                )
                            )
                        };
                        *next = Some(Rc::new(RefCell::new(link)))
                    }
                }
            }
            Self::None => { // Use case when the linked list is empty
                *self = Self::Child {
                    value: element,
                    next: None
                };
            },
        }
    }

    /// Removes the last child from the linked list
    pub fn pop(&mut self) -> () {
        match self {
            Self::Child { value: _, next } => {
                match next {
                    Some(item) => {
                        let child = &mut *item.borrow_mut();
                        match child {
                            Self::Child { value: _, next } => {
                                if next.is_none() {
                                    *child = Self::new();
                                    return;
                                }
                                child.pop()
                            },
                            _ => unreachable!()
                        }
                    }
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}

fn main() {
    println!("Linked list in rust");

    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);

    list.pop();

    println!("{:?}", list);

    list.push(5);

    println!("{:?}", list);
}
