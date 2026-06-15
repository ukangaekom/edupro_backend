use crate::models::user::{input::*, output::*};
use crate::models::exam::{input::*, output::*};
use axum::{Json,Router};
use axum::routing::{post, get};
use axum::extract::Extension;
use crate::state::AppState;
use crate::authentication::auth_middleware::AuthUser;














// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/user/dashboard", get(get_dashboard_data))
}





async fn get_dashboard_data(
    Extension(auth_user): Extension<AuthUser>,
)-> Json<user_dashboard_data>{

    println!("User id is {}", auth_user.id);

    let dashboard_data = user_dashboard_data{
        total_xps: 1000,
        rank: 1,
        total_exams_taken: 10,
        total_practices_taken: 10,
        recent_exams: vec![
            recent_exam{
                exam_id: 12345,
                exam_name: "JAMB".to_string(), 
                exam_date: "26th May, 2026".to_string(),
                score: 360,
                percent_score: 90.0,
            },
            recent_exam{
                exam_id: 12346,
                exam_name: "WAEC".to_string(), 
                exam_date: "26th June, 2026".to_string(),
                score: 350,
                percent_score: 87.5,
            },
        ],
    };

    Json(dashboard_data)
}