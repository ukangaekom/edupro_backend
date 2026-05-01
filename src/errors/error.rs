use axum::response::{IntoResponse, Response};
use serde::Serialize;
use axum::{http::StatusCode, Json};
use std::borrow::Cow;





pub type Result<T> = core::result::Result<T, Error>;


#[derive(Serialize)]
pub struct ErrorPayload{
    pub error: Cow<'static, str>,
    pub code: u16
}


#[derive(Debug)]
pub enum Error{
    LoginFail,
    RegistrationFail,
    InvalidRequest,
    NoSubscription,
    NoPermission,
}


impl IntoResponse for Error{
    fn into_response(self) -> Response{ 

        let (status, error_message) = match self {

            Self::LoginFail => (StatusCode::UNAUTHORIZED, "Login Failed"),
            Self::RegistrationFail => (StatusCode::BAD_REQUEST, "Registration Failed"),
            Self::InvalidRequest => (StatusCode::BAD_REQUEST, "Invalid Payload"),
            Self::NoSubscription => (StatusCode::FORBIDDEN, "Upgrade Your Plan"),
            Self::NoPermission => (StatusCode::FORBIDDEN, "You are not authorized to use this service"),
        };

        let error_payload = ErrorPayload{
            error: Cow::Borrowed(error_message),
            code: status.as_u16(),
        };
       
        (status,Json(error_payload)).into_response()
    }

}




// Error Handler
pub async fn error_handler() -> &'static str{
    return "The page doesn't exist";
}


