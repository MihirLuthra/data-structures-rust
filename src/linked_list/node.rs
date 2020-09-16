pub type Next<T> = Box<Option<Node<T>>>;

pub struct Node<T> {
    pub data: T,
    pub next: Next<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: Box::new(None),
        }
    }
}
