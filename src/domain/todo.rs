use crate::domain::due_to::DueTo;
use crate::domain::title::Title;

#[derive(Debug, Eq, PartialEq)]
pub struct Todo {
    pub title: Title,
    pub due_to: DueTo,
}

impl Todo {
    pub fn new(title: &Title, due_to: &DueTo) -> Self {
        Todo {
            title: title.clone(),
            due_to: due_to.clone(),
        }
    }
}
