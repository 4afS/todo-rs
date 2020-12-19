pub mod due_to;
pub mod title;
pub mod todo;

#[test]
fn create_new_todo_test() {
    let title = title::Title::new("title1");
    let due_to = due_to::DueTo::new(&chrono::Local::today().naive_local());

    let todo_new = todo::Todo::new(&title, &due_to);
    let todo_struct = todo::Todo { title, due_to };

    assert_eq!(todo_new, todo_struct)
}
