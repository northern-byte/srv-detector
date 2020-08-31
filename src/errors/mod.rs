use std::error::Error;
use serde::export::fmt::Display;
use serde::export::Formatter;
use std::fmt;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct InvalidUrlsError {
    message: &'static str,
    invalid: Vec<String>
}

impl InvalidUrlsError {
    pub(crate) fn new(invalid: Vec<String>) -> InvalidUrlsError {
        InvalidUrlsError {
            message: "Invalid urls",
            invalid
        }
    }
}

impl Display for InvalidUrlsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidUrlsError {}
