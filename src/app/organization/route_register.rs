use crate::errors::error::{Error, Result};
use serde::{Deserialize};
use axum::{Json,Router,extract::State};
use axum::routing::post;
use serde_json::{Value, json};
use axum::http::StatusCode;
use sqlx::PgPool;
use crate::state::AppState;
use crate::authentication::*;


// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/organization/register", post(api_register))
}



async fn api_register(State(state): State<AppState>, payload:Json<OrganizationRegisterPayload>) -> Result<Json<Value>>{

    println!("--> {:<12} - user api_register", "Handle");

    // TODO: Implement real db/auth logic.

    let hash = encrypt::encrypt_password(&payload.pwd);

    // let mut tx = state.db.begin().await;
    let result = sqlx::query(
        "INSERT INTO organizations (org_name, contact_email, contact_phone, password_hash) VALUES ($1,$2,$3,$4)")
        .bind(&payload.organization.as_str()).bind(&payload.contact_email)
        .bind(&payload.contact_phone).bind(&hash.await).execute(&state.db).await;


    
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
struct OrganizationRegisterPayload{
    organization:String,
    contact_email: String,
    contact_phone:String,
    pwd: String,
}