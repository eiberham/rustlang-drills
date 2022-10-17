
//! Each value in Rust has a variable that’s called its owner.
//! There can only be one owner at a time.
//! When the owner goes out of scope, the value will be dropped.

use std::fmt::{ self, Display };
use std::rc::Rc;
use std::cell::RefCell;
use core::fmt::{ Debug };

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

    pub fn pop(&mut self) -> () {
        match self {
            // Child { value: 5, next: Some(RefCell { value: Child { value: 6, next: Some(RefCell { value: None }) } }) }
            Self::Child { value: _, next } => {
                match next {
                    Some(item) => {
                        let tail = &mut *item.borrow_mut();

                        // recursively call pop only under the condition where tail.next has some value on it.

                        println!("tail {:?}", tail);

                        match tail {
                            Self::Child { value, next } => {
                                match next {
                                    Some(link) => {
                                        println!("it has a value, look: {:?}", link);
                                        let mut_s = &mut *link.borrow_mut(); // this is retrieving List<T>
                                        // I must access the value field from the enum's Child

                                        // madness here
                                        match mut_s {
                                            Self::Child { value, next } => {
                                                match next {
                                                    None => mut_s.pop(), //tail.pop()
                                                    _ => ()
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                    None => ()
                                }
                            },
                            _ => ()
                        }
                    }
                    _ => ()
                }
            }
            Self::None => {
                println!("delete the god damned node");
                // deletes the node
                *self = Self::None
            }
        }
    }
}

// a List is either Empty or has an element followed by another List

fn main() {
    println!("Linked list in rust");

    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);

    list.pop();

    println!("{:?}", list);
}
