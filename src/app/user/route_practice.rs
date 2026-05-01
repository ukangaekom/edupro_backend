use crate::models::user{input::*, output::*};
use crate::models::exams{input::*, output::*};
use axum::{Json,Router,extract::State};
use axum::extract::Extension;
use axum::routing::post;
use serde_json::{Value, json};
use sqlx::PgPool;
use crate::state::AppState;
use crate::authentication::*;


// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/user/practice/", post(start_practice))
    .route("/api/user/:practice_id/:subject_id/:question_id/", post(click_option))
}



pub async fn click_option(){
    todo!()
}