use std::fmt;

/// Custom error type for PDF processing.
///
/// This enum represents possible errors that can occur when working with PDFs:
/// - `IoError` wraps underlying I/O errors encountered during file operations.
/// - `CoreExceptionError` represents core exceptions with descriptive messages.
#[derive(Debug)]
pub enum PdfError {
    /// I/O error occurred while reading or writing a file.
    IoError(std::io::Error),

    /// A core exception occurred, described by a string message.
    CoreExceptionError(String),
}

impl fmt::Display for PdfError {
    /// Formats the error for user-friendly display.
    ///
    /// Matches on the error variant and formats a descriptive message accordingly.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PdfError::IoError(err) => write!(f, "Error reading the file: {}", err),
            PdfError::CoreExceptionError(msg) => write!(f, "Core exception error: {}", msg),
        }
    }
}

impl std::error::Error for PdfError {}
