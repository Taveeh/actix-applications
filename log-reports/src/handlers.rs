use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder, HttpResponse, HttpRequest, Result, Error, HttpMessage};
use crate::database;
use actix_files::NamedFile;
use actix_web::http::StatusCode;
use std::path::PathBuf;
use std::fs;
use crate::models::{User, AddLog};
use actix_session::{Session, UserSession};

pub async fn index(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./web/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn login(session:Session, db_pool: web::Data<Pool>, form: web::Form<User>) -> HttpResponse {
    let client: Client = db_pool.get().await.expect("Error on connecting to database");
    let result = database::login(&client, form.0.username.clone(), form.0.password).await;
    session.set("user", form.0.username.clone());
    match result {
        true => HttpResponse::Found().header("Location", "/logs").finish(),
        _ => HttpResponse::Found().header("Location", "/").finish()
    }
}

pub async fn get_log_reports(session: Session, db_pool: web::Data<Pool>) -> impl Responder {
    if get_user(session) != ""{
        let client: Client = db_pool.get().await.expect("Error on connecting to database");
        let result = database::get_log_reports(&client).await;

        match result {
            Ok(logs) => HttpResponse::Ok().json(logs),
            Err(_) => HttpResponse::InternalServerError().into()
        }
    } else {
        HttpResponse::InternalServerError().into()
    }
}

pub async fn get_css() -> impl Responder {
    let css = fs::read_to_string("./web/style.css").expect("Cannot read css");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css)
}

pub async fn log_page() -> Result<NamedFile, Error> {
    let path: PathBuf = "./web/userPage.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn get_jquery() -> impl Responder {
    let jquery = fs::read_to_string("./web/jquery-2.0.3.js").expect("Cannot read jquery");
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(jquery)
}

pub async fn get_js() -> impl Responder {
    let js = fs::read_to_string("./web/userScript.js").expect("Cannot read jquery");
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(js)
}

pub async fn get_logs_user(session: Session, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error on connecting to database");
    let current_user_session = session.get::<String>("user");
    let current_user: String;
    match current_user_session {
        Ok(user) => {
            let result = database::get_logs_user(&client, user.unwrap()).await;

            match result {
                Ok(logs) => HttpResponse::Ok().json(logs),
                Err(_) => HttpResponse::InternalServerError().into()
            }
        }
        _ => HttpResponse::InternalServerError().into()
    }

}

pub async fn get_logs_type(session: Session, db_pool: web::Data<Pool>, req: HttpRequest) -> impl Responder {
    let log_type = &req.query_string()[9..];
    println!("{}", log_type);

    if get_user(session) != "" {
        let client: Client = db_pool.get().await.expect("Error on connecting to database");
        let result = database::get_logs_type(&client, log_type.parse().unwrap()).await;

        match result {
            Ok(logs) => HttpResponse::Ok().json(logs),
            Err(_) => HttpResponse::InternalServerError().into()
        }
    } else {
        HttpResponse::InternalServerError().into()
    }
}
pub async fn get_logs_severity(session: Session, db_pool: web::Data<Pool>, req: HttpRequest) -> impl Responder {
    let severity = &req.query_string()[9..];
    println!("{}", severity);
    if get_user(session) != "" {
        let client: Client = db_pool.get().await.expect("Error on connecting to database");
        let result = database::get_logs_severity(&client, severity.parse().unwrap()).await;

        match result {
            Ok(logs) => HttpResponse::Ok().json(logs),
            Err(_) => HttpResponse::InternalServerError().into()
        }
    } else {
        HttpResponse::InternalServerError().into()
    }
}

// pub async fn add_log(session: Session, db_pool: web::Data<Pool>, req: HttpRequest) -> impl Responder {
//     let params = req.query_string()
//         .split("&");
//
//     let &mut params_array:Vec<String> = vec![];
//
//     for x in params {
//         println!("{}", x);
//         let val = x.split("=");
//         let mut cnt = 0;
//         for y in val {
//             println!("{}", y);
//             cnt = cnt + 1;
//             if cnt == 2 {
//                 params_array.append(&mut y.parse_to(String));
//             }
//         }
//     }
//     let client: Client = db_pool.get().await.expect("Error on connecting to database");
//     let current_user = get_user(session);
//     if current_user != "" {
//         // let result = database::add_log(&client, params_array[0], params_array[1], params_array[2], params_array[3], params_array[4]).await;
//
//         match result {
//             Ok(log) => HttpResponse::Ok().json(log),
//             _ => HttpResponse::InternalServerError().into()
//         }
//     } else {
//         HttpResponse::InternalServerError().into()
//     }
// }

pub async fn add_log(session: Session, db_pool: web::Data<Pool>, form: web::Form<AddLog>) -> impl Responder {
    let current_user = get_user(session);
    if current_user != "" {
        let client: Client = db_pool.get().await.expect("Error on connecting to database");
        let result = database::add_log(&client, form.0.log_type, form.0.severity, form.0.date, current_user, form.0.log).await;
        match result {
            Ok(_) => HttpResponse::Ok(),
            Err(_) => HttpResponse::InternalServerError().into()
        }
    } else {
        HttpResponse::InternalServerError().into()
    }
}

pub async fn remove_log(session: Session, db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let current_user = get_user(session);
    if current_user != "" {
        let client: Client = db_pool.get().await.expect("Error on connecting to database");
        let result = database::remove_log(&client, path.0.0, current_user).await;
        match result {
            Ok(_) => HttpResponse::Ok(),
            Err(_) => HttpResponse::InternalServerError().into()
        }
    } else {
        HttpResponse::InternalServerError().into()
    }
}

pub fn get_user(session: Session) -> String {
    let current_user_session = session.get::<String>("user");
    match current_user_session {
        Ok(user) => user.unwrap(),
        _ => "".to_string()
    }
}