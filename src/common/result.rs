use std::{fmt, string::FromUtf8Error};

// Error type
// this type incapsulates every type of error that adduce could encounter
// the primary purpose for this kind of system is twofold
// 1: upstream errors using the ? operator and reduce the use of panics
// 2: increase the verbosity and usefulness of error messages, there should be no panics

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: Option<String>,
}
impl Error {
    pub fn from_type(error_type: ErrorType, message: Option<&str>) -> Self {
        Self {
            error_type,
            message: message.map(String::from),
        }
    }
    pub fn set_msg(mut self, message: &str) -> Self {
        self.message = Some(String::from(message));
        self
    }
}

#[derive(Debug)]
pub enum ErrorType {
    FileSystem(std::io::Error),
    Dependancy(Dependancies),
    FromUtf8(FromUtf8Error),
    Toml(toml::de::Error),
    Command(String),
    CLI(CLIErrors),
    Generic(String),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ErrorType::FileSystem(error) => format!("{error}"),
            ErrorType::Dependancy(error) => format!("This program requires the program: {error}"),
            ErrorType::FromUtf8(error) => format!("{error}"),
            ErrorType::Toml(error) => format!("{error}"),
            ErrorType::Command(error) => format!("{error}"),
            ErrorType::CLI(error) => format!("{error}"),
            ErrorType::Generic(error) => format!("{error}"),
        };

        write!(f, "{message}")
    }
}

#[derive(Debug)]
pub enum CLIErrors {
    InvalidArgument,
    TooFewArguments,
}
#[derive(Debug)]
pub enum Dependancies {
    Neofetch,
    Wget,
}

impl fmt::Display for Dependancies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl fmt::Display for CLIErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

pub trait ErrorConvert<T: fmt::Debug> {
    /// Converts error type with no message
    fn res(self) -> Result<T, Error>;
    // converts error type with message
    fn res_msg(self, message: Option<&str>) -> Result<T, Error>;
    // convert error type with no return variable
    fn res_ignore(self, message: Option<&str>) -> Result<(), Error>;
}

impl<T: fmt::Debug> ErrorConvert<T> for Result<T, std::io::Error> {
    fn res(self) -> Result<T, Error> {
        self.res_msg(None)
    }
    fn res_msg(self, message: Option<&str>) -> Result<T, Error> {
        self.map_err(|error| Error::from_type(error.into(), message))
    }
    fn res_ignore(self, message: Option<&str>) -> Result<(), Error> {
        self.res_msg(message).map(|_| ())
    }
}
impl<T: fmt::Debug> ErrorConvert<T> for Result<T, Dependancies> {
    fn res(self) -> Result<T, Error> {
        self.res_msg(None)
    }
    fn res_msg(self, message: Option<&str>) -> Result<T, Error> {
        self.map_err(|error| Error::from_type(error.into(), message))
    }
    fn res_ignore(self, message: Option<&str>) -> Result<(), Error> {
        self.res_msg(message).map(|_| ())
    }
}
impl<T: fmt::Debug> ErrorConvert<T> for Result<T, FromUtf8Error> {
    fn res(self) -> Result<T, Error> {
        self.res_msg(None)
    }
    fn res_msg(self, message: Option<&str>) -> Result<T, Error> {
        self.map_err(|error| Error::from_type(error.into(), message))
    }
    fn res_ignore(self, message: Option<&str>) -> Result<(), Error> {
        self.res_msg(message).map(|_| ())
    }
}

impl<T: fmt::Debug> ErrorConvert<T> for Result<T, toml::de::Error> {
    fn res(self) -> Result<T, Error> {
        self.res_msg(None)
    }
    fn res_msg(self, message: Option<&str>) -> Result<T, Error> {
        self.map_err(|error| Error::from_type(error.into(), message))
    }
    fn res_ignore(self, message: Option<&str>) -> Result<(), Error> {
        self.res_msg(message).map(|_| ())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<std::io::Error> for ErrorType {
    fn from(val: std::io::Error) -> Self {
        ErrorType::FileSystem(val)
    }
}

impl From<FromUtf8Error> for ErrorType {
    fn from(val: FromUtf8Error) -> Self {
        ErrorType::FromUtf8(val)
    }
}

impl From<toml::de::Error> for ErrorType {
    fn from(val: toml::de::Error) -> Self {
        ErrorType::Toml(val)
    }
}

impl From<Dependancies> for ErrorType {
    fn from(val: Dependancies) -> Self {
        ErrorType::Dependancy(val)
    }
}

impl From<ErrorType> for Error {
    fn from(val: ErrorType) -> Self {
        Error {
            error_type: val,
            message: None,
        }
    }
}
