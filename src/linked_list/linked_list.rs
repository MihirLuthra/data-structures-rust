use super::errors::*;
use super::into_iter::*;
use super::iter::*;
use super::iter_mut::*;
use super::node::*;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Next<T>,
    pub length: usize,
}

// MAIN
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: Box::new(None),
            length: 0,
        }
    }
}

// INSERTION
impl<T> LinkedList<T> {
    pub fn append(&mut self, data: T) {
        self.length += 1;

        let new_node = Some(Node::new(data));

        let mut traverser = &mut self.head;

        while let Some(ref mut node) = **traverser {
            traverser = &mut (*node).next;
        }

        **traverser = new_node;
    }

    pub fn insert(&mut self, data: T, posn: usize) -> Result<(), Error> {
        self.length += 1;

        let new_node = Some(Node::new(data));
        let mut traverser = &mut self.head;
        let mut counter: usize = 0;

        while (**traverser).is_some() && counter < posn {
            counter += 1;
            traverser = &mut (**traverser).as_mut().unwrap().next;
        }

        if counter == posn {
            let prev_val = std::mem::replace(&mut **traverser, new_node);
            *((**traverser).as_mut().unwrap().next) = prev_val;
            Ok(())
        } else {
            Err(Error::PositionOutOfBounds(posn, self.length))
        }
    }

    pub fn prepend(&mut self, data: T) {
        self.length += 1;

        let new_node = Node::new(data);
        let prev_val = std::mem::replace(&mut *self.head, Some(new_node));
        if let Some(ref mut node) = *(self.head) {
            *((*node).next) = prev_val;
        }
    }
}

// DELETION
impl<T> LinkedList<T> {
    pub fn delete_at_posn(&mut self, posn: usize) -> Result<(), Error> {
        if posn >= (*self).length {
            return Err(Error::PositionOutOfBounds(posn, (*self).length));
        }

        (*self).length -= 1;

        let mut counter = 0;
        let mut iter_mut = (*self).iter_mut();

        while counter < posn {
            counter += 1;
            iter_mut.next();
        }

        iter_mut.0.map(|next| {
            **next = match std::mem::replace(&mut (**next), None) {
                Some(node) => *(node.next),
                None => None,
            };
        });

        Ok(())
    }

    pub fn delete_where<F: FnMut(&T) -> bool>(&mut self, mut f: F) -> Result<(), Error> {
        let mut iter_mut = (*self).iter_mut();
        let mut counter = 0;

        loop {
            // either the iterator contains None or iterator points to end element i.e. None
            if iter_mut.0.is_none() || (**(iter_mut.0.as_ref().unwrap())).is_none() {
                break;
            }

            let ref node_ref = *(***(iter_mut.0.as_ref().unwrap())).as_ref().unwrap();

            if f(&(*node_ref).data) {
                counter += 1;

                let deleted_node =
                    std::mem::replace(&mut (***(iter_mut.0.as_mut().unwrap())), None);
                ***(iter_mut.0.as_mut().unwrap()) = *(deleted_node.unwrap().next);
            } else {
                iter_mut.next();
            }
        }

        if counter == 0 {
            Err(Error::ElementDoesNotExist)
        } else {
            (*self).length -= counter;
            Ok(())
        }
    }
}

//OTHERS
impl<T> LinkedList<T> {
    pub fn reverse(self) -> LinkedList<T> {
        let mut reversed_linked_list = LinkedList::<T>::new();

        for element in self.into_iter() {
            reversed_linked_list.prepend(element);
        }

        reversed_linked_list
    }
}

// ITERATORS
impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(Some(&mut (*self).head))
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(&(*self).head)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self.head)
    }
}
