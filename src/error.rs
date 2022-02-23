#[derive(Debug)]
pub enum MCError {
    Parse,
    IO(std::io::Error),
    UTF8(std::str::Utf8Error),
    Stat(String),
}

impl From<std::num::ParseIntError> for MCError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::Parse
    }
}

impl From<std::num::ParseFloatError> for MCError {
    fn from(_: std::num::ParseFloatError) -> Self {
        Self::Parse
    }
}

impl From<std::io::Error> for MCError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<std::str::Utf8Error> for MCError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::UTF8(e)
    }
}

impl std::fmt::Display for MCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            MCError::Parse => write!(f, "Parse error"),
            MCError::IO(e) => write!(f, "IO error: {}", e),
            MCError::UTF8(e) => write!(f, "UTF8 error: {}", e),
            MCError::Stat(s) => write!(f, "Stat error: {}", s),
        }
    }
}
