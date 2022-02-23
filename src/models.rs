use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "todos"]
pub struct Todo {
    pub id: i32,
    pub content: String,
    pub is_done: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub content: &'a str,
    pub is_done: bool,
    pub created_at: chrono::NaiveDateTime,
}
