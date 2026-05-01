use crate::models::organization{input::*, output::*};
use axum::{Json,Router,extract::State};
use axum::routing::post;
use axum::extract::Extension;
use serde_json::{Value, json};
use sqlx::PgPool;
use crate::state::AppState;
use crate::authentication::*;





// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/organization/subscription", post(subscribe))
}


pub async fn subscribe()->{
    todo!()

    //As the payment is successful in the client side, ,kindly get the payment ID proof and subscribe in the databse
}



