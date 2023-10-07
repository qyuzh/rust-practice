use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub(crate) enum Error {
    EndOfStream,
    Other(crate::Error),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value.into())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndOfStream => "protocol error: unexpected end of stream".fmt(f),
            Self::Other(err) => err.fmt(f)
        }
    }
}

impl std::error::Error for Error {}