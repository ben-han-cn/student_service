#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate gotham;
extern crate hyper;
extern crate mime;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
