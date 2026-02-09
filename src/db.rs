use diesel::prelude::*;
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;

pub mod api {

    fn connect_db() {
        let user = dotenv::var("DB_USER");
        let password = dotenv::var("DB_PASSWORD");
        let port = dotenv::var("DB_PORT");
        let db_name = dotenv::var("DB_NAME");

        concat!();
        let url = "mysql://" + user + ":" + password + "@" + "localhost:3306/" + db_name;

        // let url: &str = "mysql://username:password@localhost:3306/database_name"
        let pool = Pool::new(url).expect("Failed to create a database pool");
        pool.get_conn().expect("Failed to get a connection")
    }

    fn get_games() {
        let user = dotenv::var("DB_USER");
        println!("db user: {:?}", user);
    }

    pub fn query_db(req: &str) {
        if req == "/spill" {
            println!("Page is Spill");
            get_games();
        }
    }
}
