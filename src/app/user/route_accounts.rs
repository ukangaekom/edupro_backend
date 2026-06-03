use crate::models::user::{input::*, output::*};
use crate::models::exam::{input::*, output::*};
use axum::{Json,Router};
use axum::routing::{post, get};
use axum::extract::Extension;
use crate::state::AppState;
use crate::authentication::auth_middleware::AuthUser;



// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/user/account", get(get_account_details))
    .route("/api/user/account/set", post(set_account_details))
}



async fn get_account_details(
    Extension(auth_user): Extension<AuthUser>,
) -> Json<user_account_details> {
    
    println!("User ID {}", auth_user.id);

    let user_details = user_account_details{
        firstname: "Ekomabasi".to_string(),
        lastname: "Ukanga".to_string(),
        email: "ekomabasiuk@gmail.com".to_string(),
        username: "ekomzy".to_string(),
        total_xps: 1000,
        rank: 1,
        total_exams_taken: 10,
        total_practices_taken: 10
        
    };

    Json(user_details)
}



async fn set_account_details(
     Extension(auth_user): Extension<AuthUser>,
)-> Json<user_account_details>{
     println!("User ID {}", auth_user.id);

    let mut user_details = user_account_details{
        firstname: "Ekomabasi".to_string(),
        lastname: "Ukanga".to_string(),
        email: "ekomabasiuk@gmail.com".to_string(),
        username: "ekomzy".to_string(),
        total_xps: 1000,
        rank: 1,
        total_exams_taken: 10,
        total_practices_taken: 10
        
    };

    Json(user_details)
}






