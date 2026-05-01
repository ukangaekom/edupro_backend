use axum::{
    response::{Response, IntoResponse}
};
use tracing::{info, warn, error, instrument};


// Main Response Mapper
pub async fn main_response_mapper(res:Response) -> impl IntoResponse {
    info!("-- Main Backend Response ---- \n\n");

    res

}