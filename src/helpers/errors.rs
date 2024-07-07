use std::{error::Error, fmt};

// Custom error type for error handling
#[derive(Debug)]
pub struct CustomError(pub String);

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl warp::reject::Reject for CustomError {}
