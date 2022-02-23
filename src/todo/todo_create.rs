use actix_web::{HttpResponse, Error, web, post};

use crate::Pool;
use crate::models::{Todo, NewTodo};
use crate::schema::todos::dsl::*;
use crate::diesel::RunQueryDsl;
use diesel::dsl::{insert_into};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoCreateInput {
    pub content: String,
}

#[post("/todo")]
pub async fn todo_create(
    db: web::Data<Pool>,
    item: web::Json<TodoCreateInput>
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_todo(db, item))
        .await
        .map(|todo| HttpResponse::Created().json(todo))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_todo(
    db: web::Data<Pool>,
    item: web::Json<TodoCreateInput>
) -> Result<Todo, diesel::result::Error> {
    let conn = db.get().unwrap();

    let new_todo = NewTodo {
        content: &item.content,
        is_done: false,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = insert_into(todos).values(&new_todo).get_result(&conn)?;

    Ok(res)
}
