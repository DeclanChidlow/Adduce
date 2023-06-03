use std::string::FromUtf8Error;

// Error type
// this type incapsulates every type of error that adduce could encounter
// the primary purpose for this kind of system is twofold
// 1: upstream errors using the ? operator and reduce the use of panics
// 2: increase the verbosity and usefulness of error messages, there should be no panics
#[derive(Debug)]
pub enum Error {
    FileSystem(std::io::Error),
    Markdown(),
    Dependancy(Dependancies),
    FromUtf8(FromUtf8Error),
    Toml(toml::de::Error),
    Command(String),
    CLI(CLIErrors),
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

pub trait ErrorConvert<T: std::fmt::Debug> {
    fn res(self) -> Result<T, Error>;
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, std::io::Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::FileSystem(error)),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, Dependancies> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Dependancy(error)),
        }
    }
}
impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, FromUtf8Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::FromUtf8(error)),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, toml::de::Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Toml(error)),
        }
    }
}
