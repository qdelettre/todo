#![feature(proc_macro_hygiene, decl_macro)]

/* Our extern crates */
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

extern crate dotenv;

/* Importing functions */
use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

/* Declaring a module, just for separating things better */
pub mod tasks;

/* Will hold our data structs */
pub mod models;

/* auto-generated table macros */
pub mod schema;

/* This will return our pg connection to use with diesel */
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    rocket::ignite().mount("/", routes![tasks::get]).launch();
}
