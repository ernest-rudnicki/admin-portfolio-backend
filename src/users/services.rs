use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, TodolistEntry};
use super::models::{CreateUserData};

// #[get("/todolist/entries")]
// async fn get_entries(data: web::Data<AppState>) -> impl Responder {
//     HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
// }

#[post("/users")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateUserData>) -> impl Responder {


    HttpResponse::Ok().json(param_obj)
}

// #[put("/todolist/entries/{id}")]
// async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, param_obj: web::Json<UpdateEntryData>) -> impl Responder {
//     let id = path.into_inner();
//     let mut todolist_entries = data.todolist_entries.lock().unwrap();

//     for i in 0..todolist_entries.len() {
//         if todolist_entries[i].id == id {
//             todolist_entries[i].title = param_obj.title.clone();
//             break;
//         }
//     }

//     HttpResponse::Ok().json(todolist_entries.to_vec())
// }

// #[delete("/todolist/entries/{id}")]
// async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
//     let mut todolist_entries = data.todolist_entries.lock().unwrap();

//     let id = path.into_inner();
//     *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id != id).collect();

//     HttpResponse::Ok().json(todolist_entries.to_vec())
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_entry);
}