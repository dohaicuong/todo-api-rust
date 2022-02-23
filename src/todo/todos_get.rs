use actix_web::{get, HttpResponse, Error, web};

use crate::Pool;
use crate::schema::todos::dsl::*;
use crate::models::Todo;
use crate::diesel::RunQueryDsl;

#[get("/todos")]
pub async fn todos_get(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_all_todos(db))
            .await
            .map(|todo| HttpResponse::Ok().json(todo))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn get_all_todos(db: web::Data<Pool>) -> Result<Vec<Todo>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = todos.load::<Todo>(&conn)?;
    Ok(items)
}
