use std::array::TryFromSliceError;
use std::char::DecodeUtf16Error;
use std::io::Error as IoError;
use std::result;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use tree_sitter::LanguageError;

#[derive(Debug)]
pub enum Error {
    DecodeUtf16Error(DecodeUtf16Error),
    Io(IoError),
    Utf8Error(Utf8Error),
    TryFromSliceError(TryFromSliceError),
    FromUtf8Error(FromUtf8Error),
    LanguageError(LanguageError),
}

impl From<DecodeUtf16Error> for Error {
    fn from(e: DecodeUtf16Error) -> Error {
        Error::DecodeUtf16Error(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::Utf8Error(e)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Error {
        Error::TryFromSliceError(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Error {
        Error::FromUtf8Error(e)
    }
}

impl From<LanguageError> for Error {
    fn from(e: LanguageError) -> Error {
        Error::LanguageError(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
