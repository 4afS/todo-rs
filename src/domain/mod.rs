pub mod due_to;
pub mod id;
pub mod title;
pub mod todo;

#[test]
fn same_id_but_different_title_and_due_to() {
    let id = id::generate();
    let title1 = title::Title::new("title1");
    let due_to1 = due_to::DueTo::new(&chrono::NaiveDate::from_ymd(2020, 1, 1));
    let title2 = title::Title::new("title2");
    let due_to2 = due_to::DueTo::new(&chrono::NaiveDate::from_ymd(2020, 1, 2));

    let todo1 = todo::Todo::new(&id, &title1, &due_to1);
    let todo2 = todo::Todo::new(&id, &title2, &due_to2);

    assert_eq!(todo1, todo2)
}
