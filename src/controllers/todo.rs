use crate::services::todo::TodoService;

use actix_web::{web, HttpResponse};
use mongodb::Database;

pub async fn get_todos(data: web::Data<Database>) -> HttpResponse {
    let todo_service = TodoService::new(data);
    let todos = todo_service.get_todos().await.unwrap();

    HttpResponse::Ok().json(todos)
}

pub async fn add_todo(
    data: web::Data<Database>,
    todo: web::Json<crate::models::todo::Todo>,
) -> HttpResponse {
    let mut todo_service = TodoService::new(data);
    todo_service.add_todo(todo.into_inner()).await;

    HttpResponse::Ok().body("Todo added successfully")
}

pub async fn update_todo(
    data: web::Data<Database>,
    todo: web::Json<crate::models::todo::CompletedTodo>,
) -> HttpResponse {
    let mut todo_service = TodoService::new(data);
    todo_service.update_todo(todo.into_inner()).await;

    HttpResponse::Ok().body("Todo updated successfully")
}
