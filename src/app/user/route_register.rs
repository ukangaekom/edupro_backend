use crate::errors::error::{Error, Result};
use serde::Deserialize;
use axum::{Json,Router,extract::State};
use axum::routing::post;
use serde_json::{Value, json};
use axum::http::StatusCode;
use sqlx::PgPool;
use crate::state::AppState;
use crate::authentication::*;






// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/user/register", post(api_register))
}



async fn api_register(State(state): State<AppState>, payload:Json<RegisterPayload>) -> Result<Json<Value>>{

    println!("--> {:<12} - user api_register", "Handle");

    // TODO: Implement real db/auth logic.

    let hash = encrypt::encrypt_password(&payload.pwd);

    // let mut tx = state.db.begin().await;
    let result = sqlx::query(
        "INSERT INTO users (firstname, lastname, email, password_hash) VALUES ($1,$2,$3,$4)")
        .bind(&payload.firstname.as_str()).bind(&payload.lastname)
        .bind(&payload.email).bind(&hash.await).execute(&state.db).await;


    
    match result{

        Ok(_)=>{

            // Create the success body 
            let body = Json(json!({
                "result":{
                "success": true
                }
            }));

            
            Ok(body)


        },

        Err(e)=>{

            // Create the success body 
            let body = Json(json!({
                "result":{
                "success": false
                }
            }));

            Ok(body)


        }
    }


}




#[derive(Debug, Deserialize)]
struct RegisterPayload{
    firstname:String,
    lastname: String,
    email:String,
    username: String,
    pwd: String,
}