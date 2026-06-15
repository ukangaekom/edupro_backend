use crate::models::user::{input::*, output::*};
use crate::models::exam::{input::*, output::*};
use crate::database::exam::{read::*, write::*};
use axum::{Json,Router,extract::State};
use axum::extract::{Path,Extension};
use axum::routing::{get,post};
use serde_json::{Value, json};
use sqlx::PgPool;
use crate::state::AppState;
use crate::authentication::auth_middleware::AuthUser;



// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/user/practice", get(get_exams_bodies))
    .route("/api/user/practice/{:extam_type_id}/subjects", get(get_exam_type_subject))
}





async fn get_exams_bodies(State(state): State<AppState>,Extension(auth_user): Extension<AuthUser>,
)-> Json<Vec<ExamType>>{
    
    let exam_types = get_exam_types(&state.db).await;
    Json(exam_types.expect("REASONS"))
}



async fn get_exam_type_subject(State(state): State<AppState>,
Path(exam_type_id): Path<i32>,
Extension(auth_user): Extension<AuthUser>,
)-> Json<Vec<ExamTypeSubject>>{
    
    let exam_subjects = get_exam_type_subjects(&state.db).await;

    Json(exam_subjects.expect("REASONS"))
}



// pub async fn click_option(Extension(auth_user): Extension<AuthUser>,
// )-> Json<selected_option>{
//     todo!()
// }