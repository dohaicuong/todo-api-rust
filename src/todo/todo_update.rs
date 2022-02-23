use actix_web::{HttpResponse, Error, web, put};

use crate::Pool;
use diesel::expression_methods::ExpressionMethods;
use crate::models::Todo;
use crate::schema::todos::dsl::*;
use crate::diesel::RunQueryDsl;
use diesel::dsl::update;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoUpdateInput {
    is_done: bool,
}

#[put("/todo/{todo_id}")]
pub async fn todo_update(
    db: web::Data<Pool>,
    todo_id: web::Path<i32>,
    item: web::Json<TodoUpdateInput>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || update_single_todo(db, todo_id.into_inner(), item))
            .await
            .map(|todo| HttpResponse::Ok().json(todo))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn update_single_todo(
    db: web::Data<Pool>,
    todo_id: i32,
    item: web::Json<TodoUpdateInput>
) -> Result<Todo, diesel::result::Error> {
    let conn = db.get().unwrap();

    let res = update(todos)
        .filter(id.eq(todo_id))
        .set(is_done.eq(item.is_done))
        .get_result(&conn)?;

    Ok(res)
}
