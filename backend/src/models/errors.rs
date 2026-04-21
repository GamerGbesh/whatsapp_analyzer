use core::fmt;
use std::io;
use axum::{
    http::StatusCode, response::{IntoResponse, Response}
};

#[derive(Debug)]
pub enum MyError{
    Io(io::Error),
    InvalidZip(zip::result::ZipError),
    EmtpyChat,
    NotFound
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(msg) => write!(f, "input: {}", msg),
            MyError::EmtpyChat => write!(f, "The chat is empty"),
            MyError::InvalidZip(msg) => write!(f, "Zip Error: {}", msg),
            MyError::NotFound => write!(f, "No whatsapp file found!")
        }
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<zip::result::ZipError> for MyError {
    fn from(err: zip::result::ZipError) -> Self {
        MyError::InvalidZip(err)
    }
}

impl IntoResponse for MyError{
    fn into_response(self) -> Response {
        let (status, message) = match self{
            MyError::EmtpyChat => (StatusCode::NO_CONTENT, "There is no chat present".to_string()),
            MyError::InvalidZip(msg) => (StatusCode::BAD_REQUEST, format!("{}", msg)),
            MyError::Io(msg) => (StatusCode::BAD_REQUEST, format!("{}", msg)),
            MyError::NotFound => (StatusCode::NOT_FOUND, "No whatsapp file found!".to_string())
        };

        (status, message).into_response()

    }
}