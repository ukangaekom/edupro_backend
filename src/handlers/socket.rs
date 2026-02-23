use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::{CookieJar,cookie::Cookie};
use futures_util::StreamExt;



// Creating the websocket Hanlder
async fn ws_handler(
    jar: CookieJar,
    ws: WebSocketUpgrade

) -> impl IntoResponse{
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await{
       
        if let Message::Text(data) = msg{

             println!("{:?}",&data.to_string());

            let _ = socket.send(Message::Text(data)).await;
        }
    }
}
