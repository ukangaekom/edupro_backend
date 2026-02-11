use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;




pub type Result<T> = core::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error{
    ConfigMissing,
}

