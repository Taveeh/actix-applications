mod model;
mod config;
mod handlers;
mod database;

use actix_web::{HttpServer, App, web, Responder};
use std::io;
use tokio_postgres::NoTls;
use dotenv::dotenv;
use crate::model::Status;
use crate::handlers::*;


#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todo_list))
            .route("/todos/{list_id}/items", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}", web::put().to(check_item))


    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
