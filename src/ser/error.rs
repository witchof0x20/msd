use serde::ser;
use std::{fmt, fmt::Display};

#[derive(Debug)]
pub enum Error {
    UnsupportedType,
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
