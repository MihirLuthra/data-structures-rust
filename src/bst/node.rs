pub type Child<T> = Box<Option<Node<T>>>;

pub struct Node<T: std::cmp::Ord> {
    pub data: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

impl<T: std::cmp::Ord> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}
