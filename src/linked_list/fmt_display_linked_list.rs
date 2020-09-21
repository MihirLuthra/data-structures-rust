use super::linked_list::*;

impl<T: std::fmt::Display> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "HEAD ->")?;
        for element in (*self).iter() {
            write!(f, " {} ->", *element)?;
        }

        write!(f, " None")?;

        Ok(())
    }
}
