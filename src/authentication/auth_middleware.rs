use axum::{
    extract::State, http::{Request, StatusCode},
    middleware::Next, response::Response,
    body::Body
};
use tower_cookies::{Cookies};
use crate::authentication::token::verify_jwt;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Clone)]
pub struct AuthUser{
    pub id: Uuid,
}




#[inline]
fn extract_auth_cookie(cookies: &str) -> Option<&str>{
    cookies.split(';').find_map(|c| {
        let c = c.trim();
        c.strip_prefix("auth=")
    })
}

pub async fn auth_middleware(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response, StatusCode>{

        let cookie = req.headers().get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .and_then(extract_auth_cookie)
            .ok_or(StatusCode::UNAUTHORIZED)?;


        let user_id = verify_jwt(cookie).await;


        req.extensions_mut().insert(AuthUser{id: user_id.expect("REASON")});

        Ok(next.run(req).await)

}