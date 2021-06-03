use actix_web::{web, Responder, HttpResponse};
use crate::model::{Status, CreateTodoList, ResultResponse};
use std::io::ErrorKind::Other;
use deadpool_postgres::{Pool, Client};
use crate::database;

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {
            status: "Ok".to_string()
        })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error on connecting to database");
    let result = database::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error on connecting to database");
    let result = database::get_items(&client, path.0.0).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn add_todo_list(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error on connecting to database");
    let result = database::create_todo_list(&client, json.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error on connecting to database");
    let result = database::check_todo_item(&client, path.0.0, path.0.1).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse {success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse {success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}