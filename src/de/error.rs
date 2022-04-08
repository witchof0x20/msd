use serde::de;
use std::{fmt, fmt::Display};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    EndOfFile,
    ExpectedTag,
    UnexpectedTag,
    EndOfTag,
    UnexpectedValues,
    UnexpectedValue,
    EndOfValues,
    ExpectedBool,
    Io,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Error {
    line: usize,
    column: usize,
    kind: Kind,
}

impl Error {
    pub(super) fn new(kind: Kind, line: usize, column: usize) -> Self {
        Self { line, column, kind }
    }
}

impl de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
