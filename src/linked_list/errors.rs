#[derive(Debug)]
pub enum Error {
    PositionOutOfBounds(usize, usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::PositionOutOfBounds(posn, length) => write!(
                f,
                "Trying to insert at {} but linked list length is {}",
                posn, length
            ),
        }
    }
}
impl std::error::Error for Error {}
