use serde::export::fmt::Display;
use serde::export::Formatter;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    BitOutOfRange(usize)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{:#?}", self))
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self where
        T: Display {
        Error::Message(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self where
        T: Display {
        Error::Message(msg.to_string())
    }
}

impl std::error::Error for Error {}