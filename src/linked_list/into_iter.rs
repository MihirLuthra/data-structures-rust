use super::node::*;

pub struct IntoIter<T>(pub Next<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = std::mem::replace(&mut (*(*self).0), None);

        current_node.map(|node| {
            (*self).0 = node.next;
            node.data
        })
    }
}
