pub mod api {
    use crate::schema::games;
    use diesel::mysql::{Mysql, MysqlConnection};
    use diesel::prelude::*;
    use dotenv::dotenv;

    #[derive(Debug, Queryable, Selectable)]
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
            "mysql://{}:{}@localhost:{}/{}",
            &user, &password, &port, &db_name,
        );

        let conn = MysqlConnection::establish(&url)
            .map_err(|e| format!("Failed to connect to the database: {}", e))?;

        println!("Successfully connected to the database");
        Ok(conn)
    }

    fn get_games() {
        let user = dotenv::var("DB_USER");
        println!("db user: {:?}", user);

        // save connection, but its waste
        match connect_db() {
            Ok(mut _conn) => {
                println!("Querying database");
                let results = games::table
                    .select(Game::as_select())
                    .load::<Game>(&mut _conn);
                println!("Games: {:?}", results);
            }
            Err(e) => {
                println!("Error connecting to database: {}", e);
            }
        }
    }

    pub fn query_db(req: &str) {
        if req == "/spill" {
            println!("Page is Spill");
            get_games();
        }
    }
}
