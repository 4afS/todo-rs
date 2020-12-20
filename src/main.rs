use chrono::Local;
use todo::{domain::{due_to::DueTo, id, repository::Repository, title::Title, todo::Todo}, interface::controller::todo::Controller, usecase::todo::Usecase};

fn main() {
    let due_to = DueTo::new(&Local::today().naive_local());
    let title = Title::new("title1");
    let id = id::generate();
    let todo = Todo::new(&id, &title, &due_to);

    print!("{:?}", todo);
}
