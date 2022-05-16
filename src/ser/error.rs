use std::{fmt::Display, io};

#[derive(thiserror::Error, Debug)]
pub enum SerError {
    #[error("Io Error: {0}")]
    Io(#[from] io::Error),
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Object key must be a string or a number")]
    KeyMustBeStringOrNumber,
}

impl serde::ser::Error for SerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}
