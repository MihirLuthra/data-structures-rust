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
        Bst::<T>::_insert(&mut (*self).root, data);
    }

    fn _insert(root: &mut Child<T>, data: T) {
        match **root {
            Some(ref mut node) => {
                if (*node).data > data {
                    Bst::<T>::_insert(&mut (*node).left, data);
                } else {
                    Bst::<T>::_insert(&mut (*node).right, data);
                }
            }
            None => {
                let new_node = Node::<T>::new(data);
                **root = Some(new_node);
            }
        };
    }
}
