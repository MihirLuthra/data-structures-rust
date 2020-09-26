use super::node::*;

pub struct Bst<T: std::cmp::Ord> {
    root: Child<T>,
}

impl<T: std::cmp::Ord> Bst<T> {
    pub fn new() -> Self {
        Bst { root: None }
    }
}
