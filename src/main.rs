// Modules
mod config;
mod errors;
mod models;
mod app;
mod state;
mod authentication;

use axum::{
    routing::{get,post},
    Router,
    middleware,
    response::{Response, IntoResponse},
    extract::{State,Path,Query},
};
use axum::http::StatusCode;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use sqlx::{PgPool, postgres::PgPoolOptions,FromRow};
use uuid::uuid;
use std::{sync::Arc,time::Duration};
use std::env;

// Crates
use crate::state::AppState;
use crate::authentication::auth_middleware;
use crate::errors::error::{error_handler};


pub use self::app::{organization, user};
pub use self::errors::error::{Error, Result};






// Main Response Mapper
async fn main_reponse_mapper(res:Response) -> impl IntoResponse {
    println!("->> {:<12} - Main_res", "RES Mapper");


    println!();

    res

}






#[tokio::main]
async fn main(){

    // Loading of Enviromental variables and connections

    dotenvy::dotenv().ok();

    let db_url= env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Creation of database connection pool
    let pool = PgPoolOptions::new()
        .min_connections(10)              // warm pool
        .max_connections(200)             // scale limit (important!)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

    // Runing of Migration
    sqlx::migrate!().run(&pool).await.expect("Migration Failed");

    let app_state = AppState { db: pool};

    // App Router initialization
    let route_all = Router::new()
        .layer(middleware::from_fn(auth_middleware::auth_middleware))
        .merge(organization::route_register::routes())
        .merge(organization::route_login::routes())
        .merge(user::route_register::routes())
        .merge(user::route_login::routes())
        .layer(middleware::map_response(main_reponse_mapper))
        .layer(CookieManagerLayer::new())
        .fallback(error_handler)
        .with_state(app_state);



    // Setting Listener
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind port 8000");


    axum::serve(listener,route_all)
        .await
        .expect("Server failed to restart");

    
}