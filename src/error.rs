use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidPath(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {

}