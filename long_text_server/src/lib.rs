use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_connection_url = env::var("DATABASE_URL").expect("NO CONNECTION");
    PgConnection::establish(&database_connection_url)
        .unwrap_or_else(|_| panic!("error with db connection"))
}


