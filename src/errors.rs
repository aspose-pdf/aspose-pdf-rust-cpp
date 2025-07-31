use std::fmt;

#[derive(Debug)]
pub enum PdfError {
    IoError(std::io::Error),
    CoreExceptionError(String),
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PdfError::IoError(err) => write!(f, "Error reading the file: {}", err),
            PdfError::CoreExceptionError(msg) => write!(f, "Core exception error: {}", msg),
        }
    }
}

impl std::error::Error for PdfError {}
