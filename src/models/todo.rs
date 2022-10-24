use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}