use chrono::Local;

use crate::domain::{due_to::DueTo, id, repository::Repository, title::Title, todo::Todo};


pub struct Db;
impl Repository for Db {
    type E = String;

    fn add(&self, todo: &Todo) -> Result<(), Self::E> {
        Ok(())
    }

    fn delete(&self, id: &id::Id) -> Result<(), Self::E> {
        Ok(())
    }

    fn get(&self, id: &id::Id) -> Result<Todo, Self::E> {
        let due_to = DueTo::new(&Local::today().naive_local());
        let title = Title::new("title1");
        let id = id::generate();
        let todo = Todo::new(&id, &title, &due_to);
        Ok(todo)
    }

    fn get_all(&self) -> Result<Vec<Todo>, Self::E> {
        let due_to = DueTo::new(&Local::today().naive_local());
        let title = Title::new("title1");
        let id = id::generate();
        let todo = Todo::new(&id, &title, &due_to);
        Ok(vec!(todo))
    }
}