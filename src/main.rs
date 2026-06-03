// Modules
mod config;
mod errors;
mod models;
mod app;
mod state;
mod authentication;
mod middlewares;
mod pipeline;

use axum::{
    routing::{get,post},
    Router,
    middleware,
    response::{Response, IntoResponse},
    extract::{State,Path,Query},
};
use axum::http::StatusCode;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_cookies::CookieManagerLayer;
use sqlx::{PgPool, postgres::PgPoolOptions,FromRow};
use uuid::uuid;
use std::{sync::Arc,time::Duration};
use std::env;

// Crates
use crate::state::AppState;
use crate::middlewares::response::main_response_mapper;
use crate::authentication::auth_middleware;
use crate::errors::error::{error_handler};
use crate::pipeline::loader::load_question_csv_to_db;


pub use self::app::{organization, user};
pub use self::errors::error::{Error, Result};




#[tokio::main]
async fn main(){

    // Loading of Enviromental variables and connections

    dotenvy::dotenv().ok();

    let db_url= env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Database URL: {}", db_url);
    // Creation of database connection pool
    let pool = PgPoolOptions::new()
        .min_connections(1)              // warm pool
        .max_connections(50)             // scale limit (important!)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

    // Runing of Migration
    sqlx::migrate!().run(&pool).await.expect("Migration Failed");


    // Tracing and Formated Subscriber
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Allow bursts with up to five requests per IP address
   // and replenishes one element every two seconds
   let governor_conf = GovernorConfigBuilder::default()
       .per_second(2)
       .burst_size(5)
       .finish()
       .unwrap();

    
    
    // Defining limiter
   let governor_limiter = Arc::new(governor_conf.limiter().clone());


   let interval = Duration::from_secs(60);

   // a separate background task to clean up
   std::thread::spawn(move || {
       loop {
           std::thread::sleep(interval);
           tracing::info!("rate limiting storage size: {}", governor_limiter.len());
           governor_limiter.retain_recent();
       }
   });

    let mut app_state = AppState { db: pool};

    let _ = load_question_csv_to_db(&mut app_state.db).await;

    // App Router initialization
    let route_all = Router::new()
        .layer(GovernorLayer{config: governor_conf.into()})
        .merge(user::route_accounts::routes())
        .merge(user::route_analytics::routes())
        .layer(middleware::from_fn(auth_middleware::auth_middleware))
        .merge(organization::route_login::routes())
        .merge(user::route_login::routes())
        .merge(user::route_register::routes())
        .merge(organization::route_register::routes())
        .layer(middleware::map_response(main_response_mapper))
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