use crate::domain::todo::Todo;
use crate::domain::id::Id;

pub trait Repository {
    type E;
    fn add(&self, todo: &Todo) -> Result<(), Self::E>;
    fn delete(&self, id: &Id) -> Result<(), Self::E>;
    fn get(&self, id: &Id) -> Result<Todo, Self::E>;
    fn get_all(&self) -> Result<Vec<Todo>, Self::E>;
}
