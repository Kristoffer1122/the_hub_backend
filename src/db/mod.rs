pub mod api {

    use crate::schema::games;
    use axum::{Json, extract::Path, response::IntoResponse};
    use diesel::mysql::{Mysql, MysqlConnection};
    use diesel::prelude::*;
    use dotenv::dotenv;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Queryable, Selectable, QueryableByName, Serialize)]
    #[diesel(table_name = games)]
    #[diesel(check_for_backend(Mysql))]
    #[allow(dead_code)]
    pub struct Table {
        pub id: i32,
        pub title: String,
        pub genre: String,
        pub image_link: Option<String>,
        pub utgivelsesdato: Option<chrono::NaiveDate>,
    }

    #[derive(Deserialize)]
    pub struct CreateGame {
        pub title: String,
        pub genre: String,
        pub image_link: Option<String>,
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

    pub async fn get_game() -> Json<Vec<Table>> {
        let mut conn = connect_db().expect("Failed to connect to DB");

        let results = games::table
            .select(Table::as_select())
            .load::<Table>(&mut conn)
            .expect("Error loading games");

        // json the data
        Json(results)
    }

    pub async fn update_game(Path(id): Path<i32>) {
        let mut conn = connect_db().expect("Failed to connect to DB");

        // Update the title of the game with id 1
        diesel::update(games::table.filter(games::id.eq(id)))
            .set(games::title.eq("Updated Title"))
            .execute(&mut conn)
            .expect("Error updating game");
    }

    pub async fn delete_game(Path(id): Path<i32>) {
        let mut conn = connect_db().expect("Failed to connect to DB");

        // Delete the game with id from path
        diesel::delete(games::table.filter(games::id.eq(id)))
            .execute(&mut conn)
            .expect("Error deleting game");
    }

    pub async fn create_game(Json(payload): Json<CreateGame>) -> impl IntoResponse {
        let mut conn = connect_db().expect("Failed to connect to DB");

        // Insert a new game into the database
        diesel::insert_into(games::table)
            .values((
                games::title.eq(payload.title),
                games::genre.eq(payload.genre),
                games::image_link.eq(payload.image_link),
                games::utgivelsesdato.eq(payload.utgivelsesdato),
            ))
            .execute(&mut conn)
            .expect("Error inserting game");
        println!("Game created successfully");
    }
}
