use std::{error::Error, fmt};
impl warp::reject::Reject for CustomError {}
use warp::reject::Rejection;
use warp::Reply;

// Custom error type for error handling
#[derive(Debug)]
pub struct CustomError(pub String);

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(custom_error) = err.find::<CustomError>() {
        let json = warp::reply::json(&serde_json::json!({
            "error": custom_error.to_string()
        }));
        Ok(warp::reply::with_status(json, warp::http::StatusCode::BAD_REQUEST))
    } else {
        // Handle other rejections (optional)
        Err(err)
    }
}
