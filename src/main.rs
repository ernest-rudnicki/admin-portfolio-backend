use actix_web:: {get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod users;
use users::services;

const HEALTH_CHECK_MSG: &str = "This is a health check";

struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> &'static str {
    HEALTH_CHECK_MSG
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
