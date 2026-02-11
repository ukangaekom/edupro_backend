use crate::errors::error::{Error, Result};
use crate::authentication::*;
use serde::{Deserialize,Serialize};
use axum::{Json, Router,response::IntoResponse,extract::State};
use serde_json::{Value, json};
use axum::routing::post;
use axum::http::StatusCode;
use tower_cookies::{Cookies};
use sqlx::{PgPool, FromRow,Row,types::Uuid};
use cookie::{Cookie,SameSite, time::Duration};
use crate::state::AppState;







// Router
pub fn routes() -> Router<AppState>{
    Router::new().route("/api/user/login", post(api_login))
}

async fn api_login(State(state): State<AppState>, cookies: Cookies, payload:Json<LoginPayload>) -> Result<Json<Value>>{



    let password_hash = encrypt::encrypt_password(&payload.pwd);


    let result = sqlx::query(
        "SELECT id, email, password_hash FROM users WHERE email = $1"
    ).bind(&payload.email).fetch_optional(&state.db)
    .await;



    if let Ok(Some(row)) = result{
        let id: Uuid = row.get("id");

        let password: String = row.get("password_hash");


            if encrypt::verify_password(&password,&payload.pwd).await{

                // Create the success body 
                let body = Json(json!({
                    "result":{
                    "success": true
                    }
                }));



                

                let token = token::create_jwt_token(&id,"free").await;
                // Building a Secure Cookie
                let cookie = Cookie::build(("auth",token))
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Strict)
                    .path("/")
                    .max_age(Duration::minutes(15))
                    .finish();
                
                cookies.add(cookie);

                println!("--> {:<12} - user api_login", "Handle");

                Ok(body)

            } else{

                 // Create the success body 
                let body = Json(json!({
                    "result":{
                    "success": false
                    }
                }));


                Ok(body)




            }

            
                
            
    }else{

         // Create the success body 
            let body = Json(json!({
                "result":{
                "success": false
                }
            }));

            
            Ok(body)




                
    }


   

}


// Implementation of api_login to be non-static


#[derive(Debug,Serialize)]
struct User{
    id: Uuid,
    password: String,
    // role: String
}


// Request


#[derive(Debug, Deserialize)]
struct LoginPayload{
    email: String,
    pwd: String,
}