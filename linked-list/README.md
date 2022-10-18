## Linked list in rust :crab:

This is a small readme I wrote about how I managed to craft a linked list in rust, as far as the implementation I have to say it's not that compelling, but given this particular data structure is normally tricky to do in rust I give myself some credit for being able to do this despite of being a beginner rustacean.

Let's see first what a linked list is:

> A linked list is an ordered set of data elements, each containing a link to its successor (and sometimes its predecessor).


Off rip, we need to find a way to represent two states in our list : empty | no empty. So in order to do that I have come up with an enum defined this way:

```shell
enum List<T> {
    None,
    Child {
        value: T,
        next: Option<Rc<RefCell<List<T>>>>
    }
}
```

Note how I'm using generics within the enum, that's because I want the linked list to hold any type of value. So we have the none state, intended to represent the empty state of our list, and a child link which will hold either another list or nothing.

So far we only have the list defined, we need to find a way to instantiate our list, right ? let's do it:

```shell
impl<T> List<T> where
    T: Debug + Display + Copy {
      /// Returns an empty instance of the list
      pub fn new() -> Self {
          Self::None
      }
    }
```

With the above new method in place from this point forward we know we can instantiate a new list by simply invoking  `List::new()`. That is lit. Now we need to define methods in order to add a new child link onto the list and remove them off of it. Let's explore first how to add elements:

```shell
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
```

Now let's add the `pop` function:

```shell
/// Removes the last child from the linked list
    pub fn pop(&mut self) -> () {
        if let Self::Child { value: _, next } = self {
            if let Some(item) = next {
                    let child = &mut *item.borrow_mut();
                        if let Self::Child { value: _, next } = child {
                            if next.is_none() {
                                *child = Self::new();
                                return;
                            }
                            child.pop()
                        }
                }
        }
    }
```

