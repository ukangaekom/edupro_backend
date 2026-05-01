use crate::models::organization{input::*, output::*};
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
    Router::new().route("/api/organization/exams", post(set_exams))
}




pub async fn set_exams()->{
    todo!()
    // Register the exams set my the organization in exam table and return 
}




