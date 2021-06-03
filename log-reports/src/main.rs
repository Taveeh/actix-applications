use std::io;
use actix_web::{HttpServer, App, web, Resource};
use dotenv::dotenv;
use actix_session::{CookieSession};
mod config;
mod handlers;
mod models;
mod database;

use crate::handlers::*;
use deadpool_postgres::tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/", web::get().to(index))
            .route("/allLogReports", web::get().to(get_log_reports))
            .route("/style.css", web::get().to(get_css))
            .route("/login", web::post().to(login))
            .route("/logs", web::get().to(log_page))
            .route("/jquery-2.0.3.js", web::get().to(get_jquery))
            .route("/userScript.js", web::get().to(get_js))
            .route("/userLogReports", web::get().to(get_logs_user))
            .route("/typeLogReports", web::get().to(get_logs_type))
            .route("/severityLogReports", web::get().to(get_logs_severity))
            .route("/addLog", web::post().to(add_log))
            .route("/deleteLog/{id}", web::delete().to(remove_log))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
