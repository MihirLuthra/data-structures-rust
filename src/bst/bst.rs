use super::node::*;

#[derive(Debug)]
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
                    } else if (*node).data < data {
                        insert(&mut (*node).right, data);
                    } else {
                        return;
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

    pub fn delete(&mut self, data: T) {
        fn delete<U: std::cmp::Ord>(root: &mut Child<U>, data: U) {
            match **root {
                Some(ref mut node) => {
                    if (*node).data > data {
                        delete(&mut (*node).left, data);
                    } else if (*node).data < data {
                        delete(&mut (*node).right, data);
                    } else {
                        /*
                         * If right node is None, simply replace the current node
                         * with its left child.
                         *
                         * If not, replace the current node with its right child
                         * and left child of the current node is made the
                         * deepest left child of right node.
                         */

                        (**root).take().map(|node| {
                            **root = *(node.right);
                            let mut traverser = &mut *root;

                            while let Some(ref mut node) = **traverser {
                                traverser = &mut (*node).left;
                            }

                            **traverser = *(node.left);
                        });
                    }
                }
                None => return,
            };
        }

        delete(&mut (*self).root, data);
    }

    pub fn search(&self, data: T) -> bool {
        fn search<U: std::cmp::Ord>(root: &Child<U>, data: U) -> bool {
            match **root {
                Some(ref node) => {
                    if (*node).data > data {
                        search(&(*node).left, data)
                    } else if (*node).data < data {
                        search(&(*node).right, data)
                    } else {
                        true
                    }
                }
                None => false,
            }
        }

        search(&(*self).root, data)
    }
}
