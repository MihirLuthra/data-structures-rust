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

#[cfg(test)]
mod test {
    use super::LinkedList;

    fn get_new_linked_list_with_values<T: Copy>(vec: &Vec<T>) -> LinkedList<T> {
        let mut linked_list = LinkedList::<T>::new();

        for element in vec.iter() {
            linked_list.append(*element);
        }

        return linked_list;
    }

    #[test]
    fn iter_mut_test() {
        let test_vector = vec![1, 2, 3, 4, 5];

        let mut linked_list = get_new_linked_list_with_values(&test_vector);

        let mut iter_mut = linked_list.iter_mut();

        for element in test_vector.iter() {
            let next = iter_mut.next().unwrap();
            *next += 1;
            assert_eq!(*element + 1, *next);
        }
    }

    #[test]
    fn iter_test() {
        let test_vector = vec![1, 2, 3, 4, 5];

        let linked_list = get_new_linked_list_with_values(&test_vector);

        let mut iter = linked_list.iter();

        for element in test_vector.iter() {
            let next = iter.next().unwrap();
            assert_eq!(*element, *next);
        }
    }

    #[test]
    fn into_iter_test() {
        let test_vector = vec![1, 2, 3, 4, 5];

        let linked_list = get_new_linked_list_with_values(&test_vector);

        let mut into_iter = linked_list.into_iter();

        for element in test_vector.iter() {
            let next = into_iter.next().unwrap();
            assert_eq!(*element, next);
        }
    }

    #[test]
    fn fmt_display_test() {
        let test_vector = vec![1, 2, 3, 4, 5];

        let linked_list = get_new_linked_list_with_values(&test_vector);

        assert_eq!(
            format!("{}", linked_list),
            "HEAD -> 1 -> 2 -> 3 -> 4 -> 5 -> None"
        );
    }

    #[test]
    fn delete_at_posn_test() {
        let test_vector = vec![1, 2, 3, 4, 5];

        let mut linked_list = get_new_linked_list_with_values(&test_vector);

        linked_list
            .delete_at_posn(2)
            .expect("Error while deleting from 2nd posn");
        assert_eq!(
            format!("{}", linked_list),
            "HEAD -> 1 -> 2 -> 4 -> 5 -> None"
        );

        linked_list
            .delete_at_posn(2)
            .expect("Error while deleting from 2nd posn");
        assert_eq!(format!("{}", linked_list), "HEAD -> 1 -> 2 -> 5 -> None");

        if let Ok(_) = linked_list.delete_at_posn(3) {
            panic!(
                "Should have caused error as we are trying to delete at a posn greater than length"
            );
        }

        assert_eq!(linked_list.length, 3);
    }
}
