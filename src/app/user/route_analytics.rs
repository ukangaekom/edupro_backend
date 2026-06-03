use crate::models::user::{input::*, output::*};
use crate::models::exam::{input::*, output::*};
use axum::{Json,Router,extract::State};
use axum::extract::{Extension,Path};
use axum::routing::post;
use serde_json::{Value, json};
use sqlx::PgPool;
use crate::state::AppState;
use crate::authentication::auth_middleware::AuthUser;


// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/user/{exam_id}/analytics", post(exam_analysis))
}




async fn exam_analysis(
    Extension(auth_user): Extension<AuthUser>,
    Path(exam_id): Path<i64>,
)-> Json<user_exam_analytics>{

    println!("User id is {}", auth_user.id);
    println!("Exam id is {}", exam_id);

    let subjects: Vec<&str> = vec!["Mathematics","English","Chemistry", "Physics"];

    let mut subject_analysis = Vec::new();

   

    for i in 0..4 {
        let exam_analytics = subject_analytics{
                exam_id: 12345,
                subject_id:i,
                subject_name:subjects[i as usize].to_string(),
                score: 90,
                total_questions:50,
                percent_score: 90.0,
            };

        subject_analysis.push(exam_analytics);

    }

     let mut exam_analytics = user_exam_analytics{
        exam_id: 12345,
        exam_name: "JAMB".to_string(), 
        exam_date: "26th May, 2026".to_string(),
        total_score: 360,
        percent_score: 90.0,
        subject_analytics: subject_analysis,

    };

    Json(exam_analytics)
}



