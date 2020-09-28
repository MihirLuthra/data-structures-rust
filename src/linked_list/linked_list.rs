use super::errors::*;
use super::into_iter::*;
use super::iter::*;
use super::iter_mut::*;
use super::node::*;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Next<T>,
    /// Get length of linked list
    pub length: usize,
}

// MAIN
impl<T> LinkedList<T> {
    /// Get new LinkedList
    pub fn new() -> Self {
        LinkedList {
            head: Box::new(None),
            length: 0,
        }
    }
}

// INSERTION
impl<T> LinkedList<T> {
    /// Append to the end of the linked list.
    ///
    /// # Time Complexity
    /// O(n)
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.append(1);
    ///  linked_list.append(2);
    ///  linked_list.append(3);
    ///
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 1 -> 2 -> 3 -> None");
    /// ```
    pub fn append(&mut self, data: T) {
        self.length += 1;

        let new_node = Some(Node::new(data));

        let mut traverser = &mut self.head;

        while let Some(ref mut node) = **traverser {
            traverser = &mut (*node).next;
        }

        **traverser = new_node;
    }

    /// Insert elemet at posn in the linked list.
    ///
    /// # Time Complexity
    /// O(posn)
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.insert(44, 0);
    ///  linked_list.insert(22, 0);
    ///  linked_list.insert(33, 1);
    ///
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 22 -> 33 -> 44 -> None");
    /// ```
    ///
    /// # Errors
    ///
    /// The function returns Error::PositionOutOfBounds(posn, len)
    /// if position to be inserted is >= linked list length.
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

    /// Add element to the beginning of the linked list.
    ///
    /// # Time Complexity
    /// O(1)
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.prepend(3);
    ///  linked_list.prepend(2);
    ///  linked_list.prepend(1);
    ///
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 1 -> 2 -> 3 -> None");
    /// ```
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
    /// Delete element at the given position.
    ///
    /// # Time Complexity
    /// O(posn)
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.append(1);
    ///  linked_list.append(2);
    ///  linked_list.append(3);
    ///  linked_list.append(4);
    ///
    ///  linked_list.delete_at_posn(2);
    ///
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 1 -> 2 -> 4 -> None");
    /// ```
    ///
    /// # Errors
    ///
    /// The function returns Error::PositionOutOfBounds(posn, len)
    /// if position to be deleted is >= linked list length.

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

    /// Delete element at the given position.
    ///
    /// # Time Complexity
    /// O(n)
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.append(1);
    ///  linked_list.append(2);
    ///  linked_list.append(3);
    ///  linked_list.append(4);
    ///
    ///  // delete first 2 elements
    ///  let mut counter = 2;
    ///  linked_list.delete_where(move |element| {
    ///     counter -= 1;
    ///     counter >= 0
    ///  });
    ///
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 3 -> 4 -> None");
    ///
    ///  linked_list.delete_where(|element| element % 2 == 0);
    ///  
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 3 -> None");
    /// ```
    ///
    /// # Errors
    ///
    /// The function returns Error::ElementDoesNotExist
    /// if no element of linked list returns true with closure f.
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
    /// Returns reversed linked list
    ///
    /// # Time Complexity
    /// O(n)
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.append(1);
    ///  linked_list.append(2);
    ///  linked_list.append(3);
    ///  linked_list.append(4);
    ///
    ///  linked_list = linked_list.reverse();
    ///
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 4 -> 3 -> 2 -> 1 -> None");
    /// ```
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
    /// Produces mutable iterator
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.append(1);
    ///  linked_list.append(2);
    ///  linked_list.append(3);
    ///  linked_list.append(4);
    ///
    ///  linked_list.iter_mut().for_each(|element| *element += 1);
    ///
    ///  assert_eq!(format!("{}", linked_list), "HEAD -> 2 -> 3 -> 4 -> 5 -> None");
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(Some(&mut (*self).head))
    }

    /// Produces am immutable iterator
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.append(1);
    ///  linked_list.append(2);
    ///  linked_list.append(3);
    ///
    ///  let mut vector = Vec::new();
    ///
    ///  for element in linked_list.iter() {
    ///     vector.push(*element);
    ///  }
    ///
    ///  assert_eq!(vector, [1, 2, 3]);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter(&(*self).head)
    }

    /// Consumes the linked list and gives an iterator.
    ///
    /// # Example
    /// ```
    ///# use crate::data_structures::linked_list::*;
    ///  let mut linked_list = LinkedList::<i32>::new();
    ///  linked_list.append(1);
    ///  linked_list.append(2);
    ///  linked_list.append(3);
    ///
    ///  let mut vector: Vec<i32> = linked_list.into_iter().collect();
    ///
    ///  assert_eq!(vector, [1, 2, 3]);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self.head)
    }
}
