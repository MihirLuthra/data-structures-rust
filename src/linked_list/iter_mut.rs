use super::node::*;

pub struct IterMut<'a, T>(pub Option<&'a mut Next<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next<'b>(&'b mut self) -> Option<&'a mut T> {
        let some_node = match (*self).0.take() {
            Some(node) => node,
            None => return None,
        };

        (**some_node).as_mut().map(|node| {
            (*self).0 = Some(&mut (*node).next);
            &mut (*node).data
        })
    }
}
