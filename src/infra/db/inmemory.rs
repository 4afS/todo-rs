use std::sync::{Arc, Mutex};

use crate::domain::{id::Id, repository::Repository, todo::Todo};

use super::error::DbError;

#[derive(Clone)]
pub struct Db {
    db: Arc<Mutex<Vec<Todo>>>,
}

impl Db {
    pub fn new() -> Self {
        Db {
            db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Repository for Db {
    type E = DbError;

    fn add(&self, todo: &Todo) -> Result<(), Self::E> {
        match self.db.lock() {
            Ok(mut v) => {
                if v.iter()
                    .map(|e| e.id.clone())
                    .collect::<Vec<_>>()
                    .contains(&todo.id)
                {
                    Err(DbError::AlreadyExists)
                } else {
                    Ok(v.push(todo.clone()))
                }
            }
            Err(e) => Err(DbError::InternalServerError(e.to_string())),
        }
    }

    fn delete(&self, id: &Id) -> Result<(), Self::E> {
        match self.db.lock() {
            Ok(mut v) => Ok(v.retain(|todo| todo.id != id.clone())),
            Err(e) => Err(DbError::InternalServerError(e.to_string())),
        }
    }

    fn get(&self, id: &Id) -> Result<Todo, Self::E> {
        match self.db.lock() {
            Ok(v) => {
                let filtered = v
                    .iter()
                    .filter(|todo| -> bool { todo.id == id.clone() })
                    .collect::<Vec<&Todo>>();

                if filtered.is_empty() {
                    Err(DbError::NotFound)
                } else {
                    Ok(filtered.get(0).unwrap().clone().clone())
                }
            }
            Err(e) => Err(DbError::InternalServerError(e.to_string())),
        }
    }

    fn get_all(&self) -> Result<Vec<Todo>, Self::E> {
        match self.db.lock() {
            Ok(v) => Ok(v.clone()),
            Err(e) => Err(DbError::InternalServerError(e.to_string())),
        }
    }
}
