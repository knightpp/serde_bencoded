use std::fmt::Display;

use serde::{de, ser};

pub type SerResult<T> = std::result::Result<T, SerError>;
pub type DeResult<T> = std::result::Result<T, DeError>;

#[derive(Debug, Clone)]
pub enum SerError {
    Message(String),
    FloatingPointNotSupported,
    Io(String),
    DictionaryKeyMustBeString,
    /// Wrapper for [`FromUtf8Error`](std::string::FromUtf8Error)
    FromUtf8Error(std::string::FromUtf8Error),
}
#[derive(Debug, Clone)]
pub enum DeError {
    Message(String),
    UnexpectedEof,
    /// (got byte, expected byte if some)
    SyntaxError(u8, Option<u8>),
    /// Wrapper for [`ParseIntegerError`](btoi::ParseIntegerError)
    ParseIntegerError(btoi::ParseIntegerError),
    /// Wrapper for [`Utf8Error`](std::str::Utf8Error)
    Utf8Error(std::str::Utf8Error),
    ExpectedString,
    ExpectedDictionary,
    ExpectedEndOfDictionary,
    ExpectedUnitStructName,
    ExpectedInteger,
    /// String with length at most 4
    ExpectedCharString,
}

impl From<std::str::Utf8Error> for DeError {
    fn from(x: std::str::Utf8Error) -> Self {
        DeError::Utf8Error(x)
    }
}

impl From<std::io::Error> for SerError {
    fn from(ioe: std::io::Error) -> Self {
        SerError::Io(ioe.to_string())
    }
}

impl From<std::string::FromUtf8Error> for SerError {
    fn from(ue: std::string::FromUtf8Error) -> Self {
        SerError::FromUtf8Error(ue)
    }
}

impl ser::Error for SerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerError::Message(msg.to_string())
    }
}

impl de::Error for DeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        DeError::Message(msg.to_string())
    }
}

impl Display for SerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerError::Message(s) => f.write_str(s),
            SerError::Io(io) => f.write_str(io),
            SerError::DictionaryKeyMustBeString => {
                f.write_str("only byte strings allowed to be keys in dictionary")
            }
            SerError::FloatingPointNotSupported => {
                f.write_str("floating point numbers are not supported")
            }
            SerError::FromUtf8Error(ue) => f.write_fmt(format_args!("{}", ue)),
        }
    }
}

impl Display for DeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeError::Message(s) => f.write_str(s),
            DeError::UnexpectedEof => f.write_str("unexpected EOF"),
            DeError::SyntaxError(expected, got) => f.write_fmt(format_args!(
                "syntax error: expected - {}, got - {:?}",
                *expected as char, got
            )),
            DeError::ParseIntegerError(pie) => f.write_fmt(format_args!("{}", pie)),
            DeError::Utf8Error(ue) => f.write_fmt(format_args!("{}", ue)),
            DeError::ExpectedString => f.write_str("expected byte string"),
            DeError::ExpectedDictionary => f.write_str("expected dictionary"),
            DeError::ExpectedEndOfDictionary => f.write_str("expected enf of dictionary"),
            DeError::ExpectedUnitStructName => f.write_str("expected name of the unit struct"),
            DeError::ExpectedCharString => {
                f.write_str("expected byte string with length at most 4 bytes")
            }
            DeError::ExpectedInteger => f.write_str("expected integer"),
        }
    }
}

impl std::error::Error for SerError {}
impl std::error::Error for DeError {}
