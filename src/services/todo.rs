use std::str::FromStr;

use actix_web::web::Data;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    Database,
};

use crate::models::todo::{CompletedTodo, Todo, TodoId};

pub struct TodoService {
    database: Data<Database>,
}

impl TodoService {
    pub fn new(database: Data<Database>) -> Self {
        TodoService { database }
    }

    pub async fn get_todos(&self) -> Result<Vec<TodoId>, Error> {
        let collection = self.database.collection::<Todo>("todos");

        let mut cursor = collection.find(doc! {}).await?;

        let mut todos = Vec::<TodoId>::new();

        while cursor.advance().await? {
            let todo = cursor.current();

            let doc = todo.to_raw_document_buf().to_document().unwrap();

            todos.push({
                TodoId {
                    id: doc.get_object_id("_id").unwrap().to_string(),
                    title: doc.get_str("title").unwrap().to_string(),
                    description: doc.get_str("description").unwrap().to_string(),
                    completed: doc.get_bool("completed").unwrap(),
                }
            })
        }

        Ok(todos)
    }

    pub async fn add_todo(&mut self, todo: Todo) {
        let collection = self.database.collection::<Todo>("todos");

        collection
            .insert_one(todo)
            .await
            .expect("Failed to insert todo into MongoDB");
    }

    pub async fn update_todo(&mut self, todo: CompletedTodo) {
        let collection = self.database.collection::<CompletedTodo>("todos");

        let object_id = ObjectId::from_str(&todo.id).unwrap();

        collection
            .update_one(
                doc! {"_id": object_id},
                doc! {"$set": {"completed": todo.completed}},
            )
            .await
            .expect("Failed to update todo in MongoDB");
    }
}
