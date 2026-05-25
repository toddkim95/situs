use std::error::Error;
use std::io;

pub(crate) type CliResult<T> = Result<T, Box<dyn Error>>;

pub(crate) fn cli_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::new(io::ErrorKind::InvalidInput, message.into()))
}
