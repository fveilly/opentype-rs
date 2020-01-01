use std::fmt::{self, Display};
use std::error;
use nom::Err;

#[derive(Debug, Clone)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new<T: Display>(message: T) -> Error {
        Error {
            message: message.to_string()
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "parse error"
    }
}

impl<I> From<Err<I>> for Error
    where I : fmt::Debug {
    fn from(err: Err<I>) -> Self {
        Error::new(format!("{:?}", err))
    }
}