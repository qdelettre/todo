use rocket_contrib::json::Json;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

/* Database data structs */
use crate::models::*;

#[get("/")]
pub fn get() -> Json<Vec<Task>> {
    let tasks: Vec<Task> = tasks::table
        .load::<Task>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    Json(tasks)
}
