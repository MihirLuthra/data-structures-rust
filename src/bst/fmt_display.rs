use super::bst::*;
use super::node::*;

impl<T: std::cmp::Ord + std::fmt::Display> std::fmt::Display for Bst<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Bst::<T>::traverse(&self.root, f)?;
        Ok(())
    }
}

impl<T: std::cmp::Ord + std::fmt::Display> Bst<T> {
    fn traverse(
        root: &Child<T>,
        f: &mut std::fmt::Formatter,
    ) -> std::result::Result<(), std::fmt::Error> {
        match **root {
            Some(ref node) => {
                Bst::<T>::traverse(&(*node).left, f)?;
                write!(f, "{} ", node.data)?;
                Bst::<T>::traverse(&(*node).right, f)?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
