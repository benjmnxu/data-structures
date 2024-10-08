#[allow(unused_imports)]
use std::{fmt::Display, mem};

#[derive(Debug)]
pub enum ListNode<T> {
    Nil,
    Cons(T, Box<ListNode<T>>),
}

impl<T> ListNode<T> {
    // Use the implementation of this method to guide your implementation of
    // `insert` and `reverse`
    /// Deletes a node from the list
    pub fn delete(&mut self) {
        // Temporarily replaces the current node with default value (Nil).
        // In exchange, we get to take ownership of the current node instead of just
        // having it by mutable reference.
        let as_owned: ListNode<T> = mem::take(self);
        match as_owned {
            ListNode::Nil => {}
            ListNode::Cons(_, next) => {
                // Write the next node to the current node
                *self = *next;
            }
        }
    }
}

// Required methods for `ListNode<T>`
impl<T> ListNode<T> {
    /// Creates a new empty list
    pub fn new() -> Self {
        ListNode::Nil
    }
    /// Inserts a new list node with value `value` after `self` and returns a reference to the new
    /// node
    pub fn insert(&mut self, value: T) -> &mut Self {
        match self {
            ListNode::Nil => {
                *self = ListNode::Cons(value, Box::new(ListNode::Nil));
                self
            }
            ListNode::Cons(_, next) => {
                let new_node = ListNode::Cons(value, Box::new(mem::take(next)));
                *next = Box::new(new_node);
                next
            }
        }
    }
    /// Reverses the list in place.
    pub fn reverse(&mut self) {
        let mut prev = ListNode::Nil;
        let mut curr = mem::take(self);

        while let ListNode::Cons(_, ref mut n) = curr {
            let next_node = mem::replace(&mut **n, prev);
            prev = curr;
            curr = next_node;
        }
        *self = prev
    }
}

// Implement `Default` for `ListNode<T>`
impl<T> Default for ListNode<T> {
    fn default() -> Self {
        ListNode::Nil
    }
}

// Implement `PartialEq` for `ListNode<T>`
impl<T: PartialEq> PartialEq for ListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ListNode::Nil, ListNode::Nil) => true,
            (ListNode::Cons(v1, n1), ListNode::Cons(v2, n2)) => v1 == v2 && n1 == n2,
            _ => false,
        }
    }
}

// Implement `Eq` for `ListNode<T>`
impl<T: Eq> Eq for ListNode<T> {}

// Implement `Display` for `ListNode<T>`
impl<T: Display> Display for ListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListNode::Nil => write!(f, "Nil"),
            ListNode::Cons(v, n) => write!(f, "{} -> {}", v, n),
        }
    }
}

// Implement `From<Vec<T>>` for `ListNode<T>`
impl<T> From<Vec<T>> for ListNode<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut list = ListNode::new();
        let mut pos = &mut list;
        for elem in vec {
            pos = pos.insert(elem);
        }
        list
    }
}
// Implement `From<ListNode<T>>` for `Vec<T>`
impl<T> From<ListNode<T>> for Vec<T> {
    fn from(list: ListNode<T>) -> Self {
        let mut vec = Vec::new();
        let mut cur = list;

        while let ListNode::Cons(v, n) = cur {
            vec.push(v);
            cur = *n;
        }

        vec
    }
}
