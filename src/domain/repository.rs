use crate::domain::todo::Todo;
use crate::domain::id::Id;

pub trait Repository<E> {
    fn add(&self, todo: &Todo) -> Result<(), E>;
    fn delete(&self, id: &Id) -> Result<(), E>;
    fn get(&self, id: &Id) -> Result<Todo, E>;
    fn get_all(&self) -> Result<Vec<Todo>, E>;
}
