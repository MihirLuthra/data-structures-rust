use super::errors::*;
use super::node::*;

pub struct LinkedList<T> {
    head: Next<T>,
    pub length: usize,
}

impl<T: std::fmt::Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: Box::new(None), length: 0 }
    }

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

        // This method works too
        /*
        let mut new_node = Node::new(data);
        std::mem::swap(&mut *(new_node.next), &mut *self.head);
        *self.head = Some(new_node);
        */
    }

    pub fn traverse(&self) {
        let mut traverser = &self.head;

        while let Some(ref node) = **traverser {
            println!("{}", (*node).data);
            traverser = &(*node).next;
        }
    }
}
