#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod schema;
mod todo;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::default();

        App::new()
            .wrap(cors)
            .data(pool.clone())
            .service(todo::todos_get::todos_get)
            .service(todo::todo_create::todo_create)
            .service(todo::todo_update::todo_update)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
