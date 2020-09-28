use super::node::*;

pub struct Bst<T: std::cmp::Ord> {
    pub root: Child<T>,
}

impl<T: std::cmp::Ord> Bst<T> {
    pub fn new() -> Self {
        Bst {
            root: Box::new(None),
        }
    }
    pub fn insert(&mut self, data: T) {
        fn insert<U: std::cmp::Ord>(root: &mut Child<U>, data: U) {
            match **root {
                Some(ref mut node) => {
                    if (*node).data > data {
                        insert(&mut (*node).left, data);
                    } else {
                        insert(&mut (*node).right, data);
                    }
                }
                None => {
                    let new_node = Node::<U>::new(data);
                    **root = Some(new_node);
                }
            };
        }
        insert(&mut (*self).root, data);
    }
}
