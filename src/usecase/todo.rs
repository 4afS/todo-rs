use crate::domain::id::Id;
use crate::domain::repository::Repository;
use crate::domain::todo::Todo;

#[derive(Clone)]
pub struct Usecase<T: Repository + Sized> {
    pub repository: T,
}

impl<T: Repository> Usecase<T> {
    pub fn create(&self, todo: &Todo) -> Result<(), T::E> {
        self.repository.add(todo)
    }
    pub fn remove(&self, id: &Id) -> Result<(), T::E> {
        self.repository.delete(id)
    }
    pub fn get(&self, id: &Id) -> Result<Todo, T::E> {
        self.repository.get(id)
    }
    pub fn all(&self) -> Result<Vec<Todo>, T::E> {
        self.repository.get_all()
    }
}
