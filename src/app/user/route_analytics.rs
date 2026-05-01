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
    Router::new().route("/api/user/:exam_id/analytics/", post(exam_analysis))
}




pub async fn exam_analysis(){

    todo!()
}



