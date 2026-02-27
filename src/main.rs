mod db;
mod schema;

use axum::{Router, routing::get, routing::post};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/getgames", get(db::api::get_games))
        .route("/games/{id}", post(db::api::update_game))
        .route("/creategame", post(db::api::create_game))
        .route("/deletegame/{id}", post(db::api::delete_game));

    // can unwrap if port is unavaliable
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();

    println!("Server running on: localhost:7878 ");

    axum::serve(listener, app).await.unwrap();
}
