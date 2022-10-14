
//! Each value in Rust has a variable that’s called its owner.
//! There can only be one owner at a time.
//! When the owner goes out of scope, the value will be dropped.

use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None
        }
    }
}

impl<T: Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
struct List<T> {
    element: Node<T>,
}

impl<T> List<T> {
    pub fn new(element: Node<T>) -> Self {
        List {
            element
        }
    }

    pub fn push(&mut self, element: Node<T>) -> () {
        match self.element.next {
            Some(_) => (),
            None => {
                self.element.next = Some(Rc::new(RefCell::new(element)))
            }
        }
    }
}

impl<T: Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.element.next {
            Some(_) => {
                println!("{}", self.element.value);
                Ok(())
            },
            None => Ok(())
        }
    }
}

// a List is either Empty or has an element followed by another List

fn main() {
    println!("Linked list in rust");

    let node: Node<i32> = Node::new(**Rc::new(RefCell::new(5).borrow_mut()));
    let mut list: List<i32> = List::new(node);

    let value : Node<i32> = Node::new(**Rc::new(RefCell::new(8).borrow_mut()));
    list.push(value);

    println!("{}", list);
}
