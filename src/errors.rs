use serde_json;
use std::fmt;
use std::io;

// A custom Error type that has one variant, "Io" representing issues
// with reading from the config file.
#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(serde_json::Error),
}

// Tell rust how to convert an IoError to a ConfigError.
// Essentially, take the original IOError and wrap it with the ConfigError.
impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::Io(error)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        ConfigError::Parse(error)
    }
}

// Tell rust how to print the ConfigError. For an IO variant, give the
// caller some context that it's config error, that it happened when trying
// to load the file and print out the original error.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Io(ref e) => write!(f, "configuration error: problem loading file: {}", e),
            ConfigError::Parse(ref e) => {
                write!(f, "configuration error: problem parsing json: {}", e)
            }
        }
    }
}

#[derive(Debug)]
pub enum SyncError {
    Io(io::Error),
}
impl From<io::Error> for SyncError {
    fn from(error: io::Error) -> Self {
        SyncError::Io(error)
    }
}
impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyncError::Io(ref e) => write!(f, "problem syncing: couldn't read local file: {}", e),
        }
    }
}
