use crate::domain::due_to::DueTo;
use crate::domain::id::Id;
use crate::domain::title::Title;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: Id,
    pub title: Title,
    pub due_to: DueTo,
}

impl PartialEq for Todo {
    fn eq(&self, that: &Todo) -> bool {
        self.id == that.id
    }
}

impl Todo {
    pub fn new(id: &Id, title: &Title, due_to: &DueTo) -> Self {
        Todo {
            id: id.clone(),
            title: title.clone(),
            due_to: due_to.clone(),
        }
    }
}
