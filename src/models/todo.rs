use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    // ** Optioal **: ObjectId
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedTodo {
    pub id: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoId {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
