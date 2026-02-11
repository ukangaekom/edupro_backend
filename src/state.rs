use sqlx::PgPool;
use std::sync::Arc;



// App State
#[derive(Clone)]
pub struct AppState{
    pub db: PgPool,
}
