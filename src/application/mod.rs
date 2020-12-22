#[cfg(test)]
use crate::domain::id::generate;

#[cfg(test)]
use self::todo::TodoResponse;

pub mod todo;

#[test]
fn todo_to_model_valid_due_to_format() {
    let id = generate();
    let req = todo::CreateTodoReqest {
        title: "title1".to_string(),
        due_to: "2020/12/22".to_string(),
    };
    assert_ne!(req.to_model(id), None);
}

#[test]
fn todo_to_model_invalid_due_to_format() {
    let id = generate();
    let req = todo::CreateTodoReqest {
        title: "title1".to_string(),
        due_to: "2020-12-22".to_string(),
    };
    assert_eq!(req.to_model(id), None);
}

#[test]
fn todo_from_model_valid_due_to_format() {
    let id = generate();
    let req = todo::CreateTodoReqest {
        title: "title1".to_string(),
        due_to: "2020/12/22".to_string(),
    };
    let model = req.to_model(id);
    assert_ne!(model, None);

    let res = TodoResponse::from_model(&model.unwrap());

    assert_eq!(id.to_string(), res.id);
    assert_eq!(req.title, res.title);
    assert_eq!(req.due_to, res.due_to);
}
