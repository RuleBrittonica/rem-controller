use std::fmt;
// use std::io;

#[derive(Debug, Clone, PartialEq)]
pub enum ControllerError {
    MakeControlsFailed,
}

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ControllerError::MakeControlsFailed => write!(f, "Failed to make controls"),
        }
    }
}

