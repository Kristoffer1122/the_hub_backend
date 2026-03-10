mod db;
mod schema;

use axum::{Router, routing::get, routing::post};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/getgames", get(db::api::get_games))
        .route("/games/{id}", post(db::api::update_game))
        .route("/creategame", post(db::api::create_game))
        .route("/deletegame/{id}", post(db::api::delete_game))
        .route("/recap/{week}/{year}", get(db::api::get_weekly_recap))
        .route("/recap/latest", get(db::api::get_latest_recap))
        .route("/saverecap", post(db::api::save_weekly_recap))
        .route("/generate-recap", post(db::api::generate_weekly_recap));

    // can unwrap if port is unavaliable
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();

    println!("Server running on: localhost:7878 ");

    axum::serve(listener, app).await.unwrap();
}
