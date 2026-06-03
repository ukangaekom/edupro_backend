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
    Router::new()
        .route("/api/user/taken_exams", post(get_user_exams))
        .route("/api/user/exams", post(practice_exams))
        .route("/api/user/{exam_id}/{subject}/questions/{question_id}", post(click_option))
        .route("/api/user/{exam_id}/submit", post(submit_exams))
        
}



pub async fn get_user_exams(
    Extension(auth_user): Extension<AuthUser>,
){
)-> Json<user_registered_exams>{
    todo!();
}



pub async fn practice_exams(
    Extension(auth_user): Extension<AuthUser>,
){
)-> Json<exams>{

    todo!();

}


pub async fn click_option(
    Extension(auth_user): Extension<AuthUser>,
    question: answer_question
)-> Json<selected_option>{
    todo!();

}


pub async fn submit_exams(
    Extension(auth_user): Extension<AuthUser>,
    session_id: submit_exams){
    todo!()
}