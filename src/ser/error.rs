use serde::ser;
use std::{fmt, fmt::Display};

/// An error that may occur during serialization.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnsupportedType,
    Io,
    Custom(String),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnsupportedType => "unsupported Rust type".fmt(formatter),
            Self::Io => "error during I/O operations".fmt(formatter),
            Self::Custom(message) => message.fmt(formatter),
        }
    }
}

impl std::error::Error for Error {}

/// An alias for a [`Result`] with the error type [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::Error;
    use serde::ser::Error as SerdeError;

    #[test]
    fn display_unsupported_type_error() {
        assert_eq!(
            format!("{}", Error::UnsupportedType),
            "unsupported Rust type"
        );
    }

    #[test]
    fn display_io_error() {
        assert_eq!(format!("{}", Error::Io), "error during I/O operations");
    }

    #[test]
    fn display_custom_error() {
        assert_eq!(
            format!("{}", Error::custom("custom error message")),
            "custom error message"
        );
    }
}
