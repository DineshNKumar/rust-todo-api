use actix_web::web;

use crate::controllers::todo::{add_todo, get_todos, update_todo};

pub fn todo_routes(cfg: &mut web::ServiceConfig) {
    // Connect to MongoDB
    cfg.service(
        web::scope("/todos")
            .route("", web::get().to(get_todos))
            .route("", web::post().to(add_todo))
            .route("", web::put().to(update_todo)),
    );
}
