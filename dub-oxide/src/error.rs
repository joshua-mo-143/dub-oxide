use std::fmt;
#[derive(Debug)]
pub enum Error {
    Hound(hound::Error),
    Symphonia(symphonia::core::errors::Error),
    IoError(std::io::Error),
    InconsistentByteLength(usize, usize),
    IncompatibleOptions(String, String),
    MissingBuilderField(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hound(err) => write!(f, "{err}"),
            Self::Symphonia(err) => write!(f, "{err}"),
            Self::IoError(err) => write!(f, "{err}"),
            Self::InconsistentByteLength(first, second) => write!(
                f,
                "Inconsistent byte length - samples total byte length: {first} - original input byte length: {second}"
            ),
            Self::IncompatibleOptions(first, second) => {
                write!(f, "Incompatible options encountered: {first}, {second}.")
            }
            Self::MissingBuilderField(str) => write!(f, "Missing builder field: {str}"),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn incompatible_options(first: &str, second: &str) -> Self {
        Self::IncompatibleOptions(first.to_string(), second.to_string())
    }

    pub fn inconsistent_byte_length(first: usize, second: usize) -> Self {
        Self::InconsistentByteLength(first, second)
    }

    pub fn missing_builder_field(field: &str) -> Self {
        Self::MissingBuilderField(field.to_string())
    }
}

impl From<hound::Error> for Error {
    fn from(value: hound::Error) -> Self {
        Self::Hound(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<symphonia::core::errors::Error> for Error {
    fn from(value: symphonia::core::errors::Error) -> Self {
        Self::Symphonia(value)
    }
}
