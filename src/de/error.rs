use crate::de::Position;
use serde::{
    de,
    de::{Expected, Unexpected},
};
use std::{fmt, fmt::Display};

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    // Formatting errors.
    EndOfFile,
    ExpectedTag,
    UnexpectedTag,
    EndOfTag,
    UnexpectedValues,
    UnexpectedValue,
    EndOfValues,

    // Value errors.
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

    // IO-related errors.
    Io,

    // User-provided errors (provided through `serde::de::Error` trait methods).
    Custom(String),
    InvalidType(String, String),
    InvalidValue(String, String),
    InvalidLength(usize, String),
    UnknownVariant(String, &'static [&'static str]),
    UnknownField(String, &'static [&'static str]),
    MissingField(&'static str),
    DuplicateField(&'static str),

    // Unrepresentable type errors.
    CannotDeserializeAsSelfDescribing,
    CannotDeserializeAsOptionInTuple,
    CannotDeserializeAsSeqInTuple,
    CannotDeserializeAsMapInTuple,
    CannotDeserializeAsStructInTuple,
    CannotDeserializeNestedStruct,
    MustDeserializeStructFieldAsIdentifier,
}

impl Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::EndOfFile => formatter.write_str("unexpected end of file"),
            Kind::ExpectedTag => formatter.write_str("expected tag"),
            Kind::UnexpectedTag => formatter.write_str("unexpected tag"),
            Kind::EndOfTag => formatter.write_str("unexpected end of tag"),
            Kind::UnexpectedValues => formatter.write_str("unexpected values"),
            Kind::UnexpectedValue => formatter.write_str("unexpected value"),
            Kind::EndOfValues => formatter.write_str("unexpected end of values"),
            Kind::ExpectedBool => formatter.write_str("expected bool"),
            Kind::ExpectedI8 => formatter.write_str("expected i8"),
            Kind::ExpectedI16 => formatter.write_str("expected i16"),
            Kind::ExpectedI32 => formatter.write_str("expected i32"),
            Kind::ExpectedI64 => formatter.write_str("expected i64"),
            Kind::ExpectedI128 => formatter.write_str("expected i128"),
            Kind::ExpectedU8 => formatter.write_str("expected u8"),
            Kind::ExpectedU16 => formatter.write_str("expected u16"),
            Kind::ExpectedU32 => formatter.write_str("expected u32"),
            Kind::ExpectedU64 => formatter.write_str("expected u64"),
            Kind::ExpectedU128 => formatter.write_str("expected u128"),
            Kind::ExpectedF32 => formatter.write_str("expected f32"),
            Kind::ExpectedF64 => formatter.write_str("expected f64"),
            Kind::ExpectedChar => formatter.write_str("expected char"),
            Kind::ExpectedString => formatter.write_str("expected string"),
            Kind::ExpectedUnit => formatter.write_str("expected unit value"),
            Kind::ExpectedIdentifier => formatter.write_str("expected identifier"),
            Kind::Io => formatter.write_str("io error"),
            Kind::Custom(msg) => formatter.write_str(msg),
            Kind::InvalidType(unexpected, expected) => {
                write!(
                    formatter,
                    "invalid type: expected {}, found {}",
                    expected, unexpected
                )
            }
            Kind::InvalidValue(unexpected, expected) => {
                write!(
                    formatter,
                    "invalid value: expected {}, found {}",
                    expected, unexpected
                )
            }
            Kind::InvalidLength(length, expected) => {
                write!(
                    formatter,
                    "invalid length {}, expected {}",
                    length, expected
                )
            }
            Kind::UnknownVariant(variant, expected) => {
                write!(
                    formatter,
                    "unknown variant {}, expected one of {:?}",
                    variant, expected
                )
            }

            Kind::UnknownField(field, expected) => {
                write!(
                    formatter,
                    "unknown field {}, expected one of {:?}",
                    field, expected
                )
            }
            Kind::MissingField(field) => {
                write!(formatter, "missing field {}", field)
            }

            Kind::DuplicateField(field) => {
                write!(formatter, "duplicate field {}", field)
            }
            Kind::CannotDeserializeAsSelfDescribing => {
                formatter.write_str("cannot deserialize as self-describing")
            }
            Kind::CannotDeserializeAsOptionInTuple => {
                formatter.write_str("cannot deserialize as option in tuple")
            }
            Kind::CannotDeserializeAsSeqInTuple => {
                formatter.write_str("cannot deserialize as seq in tuple")
            }
            Kind::CannotDeserializeAsMapInTuple => {
                formatter.write_str("cannot deserialize as map in tuple")
            }
            Kind::CannotDeserializeAsStructInTuple => {
                formatter.write_str("cannot deserialize as struct in tuple")
            }
            Kind::CannotDeserializeNestedStruct => {
                formatter.write_str("cannot deserialize nested struct")
            }
            Kind::MustDeserializeStructFieldAsIdentifier => {
                formatter.write_str("must deserialize struct field as identifier")
            }
        }
    }
}

/// An error that may occur during deserialization.
#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    position: Position,
    kind: Kind,
}

impl Error {
    pub(super) fn new(kind: Kind, position: Position) -> Self {
        Self { position, kind }
    }

    pub(in crate::de) fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::new(Kind::Custom(msg.to_string()), Position::new(0, 0))
    }

    fn invalid_type(unexpected: Unexpected, expected: &dyn Expected) -> Self {
        Self::new(
            Kind::InvalidType(unexpected.to_string(), expected.to_string()),
            Position::new(0, 0),
        )
    }

    fn invalid_value(unexpected: Unexpected, expected: &dyn Expected) -> Self {
        Self::new(
            Kind::InvalidValue(unexpected.to_string(), expected.to_string()),
            Position::new(0, 0),
        )
    }

    fn invalid_length(len: usize, expected: &dyn Expected) -> Self {
        Self::new(
            Kind::InvalidLength(len, expected.to_string()),
            Position::new(0, 0),
        )
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Self::new(
            Kind::UnknownVariant(variant.to_owned(), expected),
            Position::new(0, 0),
        )
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self::new(
            Kind::UnknownField(field.to_owned(), expected),
            Position::new(0, 0),
        )
    }

    fn missing_field(field: &'static str) -> Self {
        Self::new(Kind::MissingField(field), Position::new(0, 0))
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self::new(Kind::DuplicateField(field), Position::new(0, 0))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{} at line {} column {}",
            self.kind,
            self.position.line(),
            self.position.column()
        )
    }
}

impl std::error::Error for Error {}

/// An alias for a [`Result`] with the error type [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::{Error, Kind};
    use crate::de::Position;
    use serde::de::Error as SerdeError;
    use serde::de::Unexpected;

    #[test]
    fn end_of_file() {
        assert_eq!(
            format!("{}", Error::new(Kind::EndOfFile, Position::new(1, 2))),
            "unexpected end of file at line 1 column 2"
        );
    }

    #[test]
    fn expected_tag() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedTag, Position::new(2, 3))),
            "expected tag at line 2 column 3"
        );
    }

    #[test]
    fn unexpected_tag() {
        assert_eq!(
            format!("{}", Error::new(Kind::UnexpectedTag, Position::new(3, 4))),
            "unexpected tag at line 3 column 4"
        );
    }

    #[test]
    fn end_of_tag() {
        assert_eq!(
            format!("{}", Error::new(Kind::EndOfTag, Position::new(4, 5))),
            "unexpected end of tag at line 4 column 5"
        );
    }

    #[test]
    fn unexpected_values() {
        assert_eq!(
            format!(
                "{}",
                Error::new(Kind::UnexpectedValues, Position::new(5, 6))
            ),
            "unexpected values at line 5 column 6"
        );
    }

    #[test]
    fn unexpected_value() {
        assert_eq!(
            format!("{}", Error::new(Kind::UnexpectedValue, Position::new(6, 7))),
            "unexpected value at line 6 column 7"
        );
    }

    #[test]
    fn end_of_values() {
        assert_eq!(
            format!("{}", Error::new(Kind::EndOfValues, Position::new(7, 8))),
            "unexpected end of values at line 7 column 8"
        );
    }

    #[test]
    fn expected_bool() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedBool, Position::new(8, 9))),
            "expected bool at line 8 column 9"
        );
    }

    #[test]
    fn expected_i8() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedI8, Position::new(9, 10))),
            "expected i8 at line 9 column 10"
        );
    }

    #[test]
    fn expected_i16() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedI16, Position::new(10, 11))),
            "expected i16 at line 10 column 11"
        );
    }

    #[test]
    fn expected_i32() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedI32, Position::new(11, 12))),
            "expected i32 at line 11 column 12"
        );
    }

    #[test]
    fn expected_i64() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedI64, Position::new(12, 13))),
            "expected i64 at line 12 column 13"
        );
    }

    #[test]
    fn expected_i128() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedI128, Position::new(13, 14))),
            "expected i128 at line 13 column 14"
        );
    }

    #[test]
    fn expected_u8() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedU8, Position::new(14, 15))),
            "expected u8 at line 14 column 15"
        );
    }

    #[test]
    fn expected_u16() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedU16, Position::new(15, 16))),
            "expected u16 at line 15 column 16"
        );
    }

    #[test]
    fn expected_u32() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedU32, Position::new(16, 17))),
            "expected u32 at line 16 column 17"
        );
    }

    #[test]
    fn expected_u64() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedU64, Position::new(17, 18))),
            "expected u64 at line 17 column 18"
        );
    }

    #[test]
    fn expected_u128() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedU128, Position::new(18, 19))),
            "expected u128 at line 18 column 19"
        );
    }

    #[test]
    fn expected_f32() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedF32, Position::new(19, 20))),
            "expected f32 at line 19 column 20"
        );
    }

    #[test]
    fn expected_f64() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedF64, Position::new(20, 21))),
            "expected f64 at line 20 column 21"
        );
    }

    #[test]
    fn expected_char() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedChar, Position::new(21, 22))),
            "expected char at line 21 column 22"
        );
    }

    #[test]
    fn expected_string() {
        assert_eq!(
            format!(
                "{}",
                Error::new(Kind::ExpectedString, Position::new(22, 23))
            ),
            "expected string at line 22 column 23"
        );
    }

    #[test]
    fn expected_unit() {
        assert_eq!(
            format!("{}", Error::new(Kind::ExpectedUnit, Position::new(23, 24))),
            "expected unit value at line 23 column 24"
        );
    }

    #[test]
    fn expected_identifier() {
        assert_eq!(
            format!(
                "{}",
                Error::new(Kind::ExpectedIdentifier, Position::new(24, 25))
            ),
            "expected identifier at line 24 column 25"
        );
    }

    #[test]
    fn io() {
        assert_eq!(
            format!("{}", Error::new(Kind::Io, Position::new(25, 26))),
            "io error at line 25 column 26"
        );
    }

    #[test]
    fn custom() {
        let mut error = Error::custom("foo");
        error.set_position(Position::new(26, 27));

        assert_eq!(format!("{}", error), "foo at line 26 column 27");
    }

    #[test]
    fn invalid_type() {
        let mut error = Error::invalid_type(Unexpected::Bool(true), &"foo");
        error.set_position(Position::new(27, 28));

        assert_eq!(
            format!("{}", error),
            "invalid type: expected foo, found boolean `true` at line 27 column 28"
        );
    }

    #[test]
    fn invalid_value() {
        let mut error = Error::invalid_value(Unexpected::Bool(true), &"foo");
        error.set_position(Position::new(28, 29));

        assert_eq!(
            format!("{}", error),
            "invalid value: expected foo, found boolean `true` at line 28 column 29"
        );
    }

    #[test]
    fn invalid_length() {
        let mut error = Error::invalid_length(42, &"foo");
        error.set_position(Position::new(29, 30));

        assert_eq!(
            format!("{}", error),
            "invalid length 42, expected foo at line 29 column 30"
        );
    }

    #[test]
    fn unknown_variant() {
        static EXPECTED: &'static [&'static str] = &["foo", "bar"];
        let mut error = Error::unknown_variant("baz", EXPECTED);
        error.set_position(Position::new(30, 31));

        assert_eq!(
            format!("{}", error),
            "unknown variant baz, expected one of [\"foo\", \"bar\"] at line 30 column 31"
        );
    }

    #[test]
    fn unknown_field() {
        let mut error = Error::unknown_field("baz", &["foo", "bar"]);
        error.set_position(Position::new(31, 32));

        assert_eq!(
            format!("{}", error),
            "unknown field baz, expected one of [\"foo\", \"bar\"] at line 31 column 32"
        );
    }

    #[test]
    fn missing_field() {
        let mut error = Error::missing_field("foo");
        error.set_position(Position::new(32, 33));

        assert_eq!(
            format!("{}", error),
            "missing field foo at line 32 column 33"
        );
    }

    #[test]
    fn duplicate_field() {
        let mut error = Error::duplicate_field("foo");
        error.set_position(Position::new(33, 34));

        assert_eq!(
            format!("{}", error),
            "duplicate field foo at line 33 column 34"
        );
    }

    #[test]
    fn cannot_deserialize_as_self_describing() {
        assert_eq!(
            format!(
                "{}",
                Error::new(
                    Kind::CannotDeserializeAsSelfDescribing,
                    Position::new(34, 35)
                )
            ),
            "cannot deserialize as self-describing at line 34 column 35"
        );
    }

    #[test]
    fn cannot_deserialize_as_option_in_tuple() {
        assert_eq!(
            format!(
                "{}",
                Error::new(
                    Kind::CannotDeserializeAsOptionInTuple,
                    Position::new(35, 36)
                )
            ),
            "cannot deserialize as option in tuple at line 35 column 36"
        );
    }

    #[test]
    fn cannot_deserialize_as_seq_in_tuple() {
        assert_eq!(
            format!(
                "{}",
                Error::new(Kind::CannotDeserializeAsSeqInTuple, Position::new(36, 37))
            ),
            "cannot deserialize as seq in tuple at line 36 column 37"
        );
    }

    #[test]
    fn cannot_deserialize_as_map_in_tuple() {
        assert_eq!(
            format!(
                "{}",
                Error::new(Kind::CannotDeserializeAsMapInTuple, Position::new(37, 38))
            ),
            "cannot deserialize as map in tuple at line 37 column 38"
        );
    }

    #[test]
    fn cannot_deserialize_as_struct_in_tuple() {
        assert_eq!(
            format!(
                "{}",
                Error::new(
                    Kind::CannotDeserializeAsStructInTuple,
                    Position::new(38, 39)
                )
            ),
            "cannot deserialize as struct in tuple at line 38 column 39"
        );
    }

    #[test]
    fn cannot_deserialize_nested_struct() {
        assert_eq!(
            format!(
                "{}",
                Error::new(Kind::CannotDeserializeNestedStruct, Position::new(39, 40))
            ),
            "cannot deserialize nested struct at line 39 column 40"
        );
    }

    #[test]
    fn must_deserialize_struct_field_as_identifier() {
        assert_eq!(
            format!(
                "{}",
                Error::new(
                    Kind::MustDeserializeStructFieldAsIdentifier,
                    Position::new(40, 41)
                )
            ),
            "must deserialize struct field as identifier at line 40 column 41"
        );
    }

    #[test]
    fn set_position() {
        let mut error = Error::new(Kind::EndOfFile, Position::new(0, 0));

        error.set_position(Position::new(1, 2));

        assert_eq!(error.position, Position::new(1, 2));
    }
}
