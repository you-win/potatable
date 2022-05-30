use std::fmt::{Debug, Display, Formatter};

pub enum Error {
    Generic(String)
}

pub type Result<T> = std::result::Result<T, Error>;

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(msg) => f.debug_tuple("Generic").field(msg).finish()
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(msg) => Display::fmt(msg, f)
        }
    }
}
