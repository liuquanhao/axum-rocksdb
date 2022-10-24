use super::todo::{CreateTodo, Todo, UpdateTodo};
use crate::errors::todo_error::TodoRepoError;
use crate::kv::{RocksDB, KVStore};

use axum::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use serde_json;

pub type DynTodoRepo = Arc<dyn TodoRepoTrait + Send + Sync>;

pub struct TodoRepo {
    pub client: RocksDB,
}

#[async_trait]
pub trait TodoRepoTrait {
    async fn create_todo(&self, create_todo: CreateTodo) -> Result<Uuid, TodoRepoError>;
    async fn delete_todo(&self, id: Uuid) -> Result<(), TodoRepoError>;
    async fn update_todo(&self, id: Uuid, update_todo: UpdateTodo) -> Result<(), TodoRepoError>;
    async fn get_todo(&self, id: Uuid) -> Result<String, TodoRepoError>;
}

#[async_trait]
impl TodoRepoTrait for TodoRepo {
    async fn create_todo(&self, create_todo: CreateTodo) -> Result<Uuid, TodoRepoError> {
        let id = Uuid::new_v4();
        let todo = Todo {
            id: id.clone(),
            text: create_todo.text,
            completed: false,
        };
        if self.client.save(&id.hyphenated().to_string(), &serde_json::to_string(&todo).unwrap()).await {
            Ok(id)
        } else {
            Err(TodoRepoError::DatabaseError)
        }

    }

    async fn delete_todo(&self, id: Uuid) -> Result<(), TodoRepoError> {
        if self.client.delete(&id.hyphenated().to_string()).await {
            Ok(())
        } else {
            Err(TodoRepoError::DatabaseError)
        }
    }

    async fn update_todo(&self, id: Uuid, update_todo: UpdateTodo) -> Result<(), TodoRepoError> {
        let todo = Todo {
            id,
            text: update_todo.text,
            completed: update_todo.completed
        };
        if self.client.save(&id.hyphenated().to_string(), &serde_json::to_string(&todo).unwrap()).await {
            Ok(())
        } else {
            Err(TodoRepoError::DatabaseError)
        }
    }

    async fn get_todo(&self, id: Uuid) -> Result<String, TodoRepoError> {
        match self.client.find(&id.hyphenated().to_string()).await {
            Some(str) => Ok(str),
            None => Err(TodoRepoError::DatabaseError)
        }
    }

}

impl TodoRepo {
    pub async fn new(client: RocksDB) -> Self {
        TodoRepo {
            client
        }
    }

    pub fn to_dyn(self) -> DynTodoRepo {
        Arc::new(self) as DynTodoRepo
    }
}