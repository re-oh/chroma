use std::fmt;

#[derive(Debug)]
pub enum EditorError {
  FilePickerClosed,
  IoError(std::io::Error),
}

impl fmt::Display for EditorError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      EditorError::FilePickerClosed => write!(f, "File picker dialog was closed"),
      EditorError::IoError(err) => write!(f, "IO error: {}", err),
    }
  }
}

impl std::error::Error for EditorError {}

impl From<std::io::Error> for EditorError {
  fn from(err: std::io::Error) -> Self {
    EditorError::IoError(err)
  }
}

pub type Result<T> = std::result::Result<T, EditorError>;