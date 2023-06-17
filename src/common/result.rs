use std::{fmt, string::FromUtf8Error};

// Error type
// this type incapsulates every type of error that adduce could encounter
// the primary purpose for this kind of system is twofold
// 1: upstream errors using the ? operator and reduce the use of panics
// 2: increase the verbosity and usefulness of error messages, there should be no panics
#[derive(Debug)]
pub enum Error {
    FileSystem(std::io::Error),
    Dependancy(Dependancies),
    FromUtf8(FromUtf8Error),
    Toml(toml::de::Error),
    Command(String),
    CLI(CLIErrors),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Error::FileSystem(error) => format!("{error}"),
            Error::Dependancy(error) => format!("This program requires the program: {error}"),
            Error::FromUtf8(error) => format!("{error}"),
            Error::Toml(error) => format!("{error}"),
            Error::Command(error) => format!("{error}"),
            Error::CLI(error) => format!("{error}"),
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
    fn res(self) -> Result<T, Error>;
}

impl<T: fmt::Debug> ErrorConvert<T> for Result<T, std::io::Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::FileSystem(error)),
        }
    }
}

impl<T: fmt::Debug> ErrorConvert<T> for Result<T, Dependancies> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Dependancy(error)),
        }
    }
}
impl<T: fmt::Debug> ErrorConvert<T> for Result<T, FromUtf8Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::FromUtf8(error)),
        }
    }
}

impl<T: fmt::Debug> ErrorConvert<T> for Result<T, toml::de::Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Toml(error)),
        }
    }
}
