use chrono::Local;
use todo::domain::{due_to::DueTo, title::Title, todo::Todo};

fn main() {
    let due_to = DueTo::new(&Local::today().naive_local());
    let title = Title::new("title1");
    let todo = Todo::new(&title, &due_to);
    print!("{:?}", todo);
}
