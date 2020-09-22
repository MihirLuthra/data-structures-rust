#[derive(Debug)]
pub enum Error {
    PositionOutOfBounds(usize, usize),
    ElementDoesNotExist,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::PositionOutOfBounds(posn, length) => write!(
                f,
                "Trying to access at {} but list's length is {}",
                posn, length
            ),

            Error::ElementDoesNotExist => write!(f, "Element doesn't exist"),
        }
    }
}
impl std::error::Error for Error {}
