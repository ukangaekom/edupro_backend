use crate::models::organization{input::*, output::*};
use crate::models::exams{input::*, output::*};
use axum::{Json,Router,extract::State};
use axum::routing::post;
use axum::extract::Extension;
use serde_json::{Value, json};
use sqlx::PgPool;
use crate::state::AppState;
use crate::authentication::*;









// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/organization/set/question", post(set_question))
    .route("/api/organization/get/questions", post(get_questions))
}







pub async set_question(){
    todo!();
    // Kindly register the question instructions in the database
}



async fn get_question(){
    todo!()
    //Kindly get the questions owned by the organization from the database.

}