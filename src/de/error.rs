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
    ExpectedI8,
    ExpectedI16,
    ExpectedI32,
    ExpectedI64,
    ExpectedI128,
    ExpectedU8,
    ExpectedU16,
    ExpectedU32,
    ExpectedU64,
    ExpectedU128,
    ExpectedF32,
    ExpectedF64,
    ExpectedChar,
    ExpectedString,
    ExpectedUnit,
    ExpectedIdentifier,
    Io,
    Custom,
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
        // TODO: FIX THIS!
        // Need a way to provide the position to the user-provided error messages.
        // Perhaps injecting the position into the error after it is returned from user code?
        // Also need a way to include the custom error messages. That doesn't jive with this struct
        // being Copy.
        Self::new(Kind::Custom, 0, 0)
    }
}

impl Display for Error {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
