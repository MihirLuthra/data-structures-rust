use super::errors::*;
use super::iter_mut::*;
use super::iter::*;
use super::into_iter::*;
use super::node::*;

pub struct LinkedList<T> {
    head: Next<T>,
    pub length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: Box::new(None),
            length: 0,
        }
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

    fn get_new_linked_list_with_values<T: Copy>(vec: & Vec<T>) -> LinkedList::<T> {
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

}
