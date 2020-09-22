/* Import macros and others */
use crate::schema::*;

#[table_name = "tasks"]
#[derive(serde::Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}
