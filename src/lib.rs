#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn db_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
