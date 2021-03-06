use std::path::PathBuf;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

use tera;
use toml;

/// A crate private constructor for `Error`.
pub(crate) fn new_error(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

/// A type alias for `Result<T, kickstart::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when using kickstart
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

/// The specific type of an error.
#[derive(Debug)]
pub enum ErrorKind {
    MissingTemplateDefinition,
    InvalidTemplate,
    UnreadableStdin,
    /// An error while cloning a repository
    Git { err: io::Error },
    /// An error while doing IO (reading/writing files)
    Io { err: io::Error, path: PathBuf },
    /// An error when rendering a template, where a template can also refer to a filename
    /// in the case kickstart
    Tera { err: tera::Error, path: Option<PathBuf> },
    /// An error while deserializing a template.toml into a struct
    Toml { err: toml::de::Error },
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        new_error(ErrorKind::Io { err, path: PathBuf::new() })
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Io { ref err, .. } => err.description(),
            ErrorKind::Tera { ref err, .. } => err.description(),
            ErrorKind::InvalidTemplate => "invalid template",
            ErrorKind::MissingTemplateDefinition => "missing template.toml",
            ErrorKind::UnreadableStdin => "couldn't read from stdin",
            ErrorKind::Git { ref err } => err.description(),
            ErrorKind::Toml { ref err } => err.description(),
            _ => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self.0 {
            ErrorKind::Io { ref err, .. } => Some(err),
            ErrorKind::Tera { ref err, .. } => Some(err),
            ErrorKind::Toml { ref err } => Some(err),
            ErrorKind::Git { ref err } => Some(err),
            ErrorKind::InvalidTemplate => None,
            ErrorKind::MissingTemplateDefinition => None,
            ErrorKind::UnreadableStdin => None,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io { ref err, ref path } => write!(f, "{}: {:?}", err, path),
            ErrorKind::Tera { ref err, ref path } => {
                if let Some(p) = path {
                    write!(f, "{}: {:?}", err, p)
                } else {
                    write!(f, "{}: rendering a one-off template", err)
                }
            }
            ErrorKind::Git { ref err } => write!(f, "Could not clone the repository: {}", err),
            ErrorKind::Toml { ref err } => write!(f, "Invalid TOML: {}", err),
            ErrorKind::MissingTemplateDefinition => write!(f, "The template.toml is missing"),
            ErrorKind::UnreadableStdin => write!(f, "Unable to read from stdin"),
            ErrorKind::InvalidTemplate => write!(f, "The template.toml is invalid"),
            _ => unreachable!(),
        }
    }
}
