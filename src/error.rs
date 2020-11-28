use std::{fmt::Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Message(String),
    FloatingPointNotSupported,
    Io(std::rc::Rc<std::io::Error>),
    DictionaryKeyMustBeString,
    FromUtf8Error(std::string::FromUtf8Error),
}

impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl de::Error for Error{
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(s) => f.write_str(s),
            Error::Io(_) => todo!(),
            Error::DictionaryKeyMustBeString => todo!(),
            Error::FloatingPointNotSupported => todo!(),
            Error::FromUtf8Error(_) => todo!(),
        }
    }
}

impl std::error::Error for Error {}
