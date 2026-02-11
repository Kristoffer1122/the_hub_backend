pub mod api {

    use crate::schema::games;
    use axum::Json;
    use diesel::mysql::{Mysql, MysqlConnection};
    use diesel::prelude::*;
    use dotenv::dotenv;
    use serde::Serialize;

    #[derive(Debug, Queryable, Selectable, QueryableByName, Serialize)]
    #[diesel(table_name = games)]
    #[diesel(check_for_backend(Mysql))]
    #[allow(dead_code)]
    pub struct Game {
        pub id: i32,
        pub title: String,
        pub genre: String,
        pub utgivelsesdato: Option<chrono::NaiveDate>,
    }

    pub fn connect_db() -> Result<MysqlConnection, Box<dyn std::error::Error>> {
        println!("Checking database...");
        dotenv().ok();

        let user = std::env::var("DB_USER")?;
        let password = std::env::var("DB_PASSWORD")?;
        let port = std::env::var("DB_PORT")?;
        let db_name = std::env::var("DB_NAME")?;

        let url = format!(
            "mysql://{}:{}@host.docker.internal:{}/{}",
            &user, &password, &port, &db_name,
        );

        let conn = MysqlConnection::establish(&url)
            .map_err(|e| format!("Failed to connect to the database: {}", e))?;

        println!("Successfully connected to the database");
        Ok(conn)
    }

    pub async fn get_games() -> Json<Vec<Game>> {
        let mut conn = connect_db().expect("Failed to connect to DB");

        let results = games::table
            .select(Game::as_select())
            .load::<Game>(&mut conn)
            .expect("Error loading games");

        Json(results) // ← This is like res.json(results) in Express
    }
}
