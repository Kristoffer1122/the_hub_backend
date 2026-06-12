mod db;
mod schema;

use axum::{Router, routing::get, routing::post};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/recap/{year}/{week}", get(db::api::get_weekly_recap))
        .route("/recap/latest", get(db::api::get_latest_recap))
        .route("/saverecap", post(db::api::save_weekly_recap))
        .route("/generate-recap", post(db::api::generate_weekly_recap));

    // can unwrap if port is unavaliable
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();

    println!("Server running on: localhost:7878 ");

    axum::serve(listener, app).await.unwrap();
}
