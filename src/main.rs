mod db;
mod schema;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/games", get(db::api::get_games));

    // can unwrap if port is unavaliable
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();

    println!("Server running on: localhost:7878 ");

    axum::serve(listener, app).await.unwrap();
}
