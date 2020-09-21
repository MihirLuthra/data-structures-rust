use super::node::*;

pub struct Iter<'a, T>(pub &'a Next<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next<'b>(&'b mut self) -> Option<&'a T> {
        (**(*self).0).as_ref().map(|node| {
            (*self).0 = & (*node).next;
            & (*node).data
        })
    }
}
