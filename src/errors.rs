use std::error::Error;
use serde::export::fmt::Display;
use serde::export::Formatter;
use std::fmt;

#[derive(Debug)]
pub struct InvalidUrlsError {
    message: &'static str
}

impl InvalidUrlsError {
    pub(crate) fn new() -> InvalidUrlsError {
        InvalidUrlsError {
            message: "Invalid urls"
        }
    }
}

impl Display for InvalidUrlsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidUrlsError {}
