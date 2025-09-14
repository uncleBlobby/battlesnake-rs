#![allow(non_snake_case)]

mod handlers;
mod models;

use axum::{
    Router,
    routing::{get, post},
};

use handlers::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_battlesnake_details))
        .route("/start", post(game_start_handler))
        .route("/move", post(move_handler))
        .route("/end", post(game_end_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
