pub mod api {
    use diesel::mysql::MysqlConnection;
    use diesel::prelude::*;
    use dotenv::dotenv;

    pub fn connect_db() -> Result<MysqlConnection, Box<dyn std::error::Error>> {
        dotenv().ok();

        let user = dotenv::var("DB_USER");
        let password = dotenv::var("DB_PASSWORD");
        let port = dotenv::var("DB_PORT");
        let db_name = dotenv::var("DB_NAME");

        let url = format!(
            "mysql://{}:{}@localhost:{}/{}",
            user?, password?, port?, db_name?,
        );
        println!("Database connection vars: {:#?}", url);

        let conn = MysqlConnection::establish(&url)
            .map_err(|e| format!("Failed to connect to the database: {}", e))?;

        println!("Successfully connected to the database");
        Ok(conn)
    }

    fn get_games() {
        let user = dotenv::var("DB_USER");
        println!("db user: {:?}", user);

        match connect_db() {
            Ok(_conn) => {
                println!("Connected to the database successfully!");
                // You can perform database operations here using `conn`
            }
            Err(e) => {
                println!("Failed to connect to the database: {}", e);
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
