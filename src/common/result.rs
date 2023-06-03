// Error type
// this type incapsulates every type of error that adduce could encounter
// the primary purpose for this kind of system is twofold
// 1: upstream errors using the ? operator and reduce the use of panics
// 2: increase the verbosity and usefulness of error messages, there should be no panics
pub enum Error {
    FileSystem(std::io::Error),
    Markdown(),
}

// traits
// todo implement trait for every error type
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
