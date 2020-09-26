pub type Child<T> = Option<Box<Node<T>>>;

pub struct Node<T: std::cmp::Ord> {
    data: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T: std::cmp::Ord> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}
