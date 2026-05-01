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
    Router::new().route("/api/organization/account", post(get_account))
}



async pub fn get_account(){
    todo!()

    // fetch user accout via ID and return
}


