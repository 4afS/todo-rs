use crate::domain::{
    due_to::DueTo,
    id::{generate, Id},
    repository::Repository,
    title::Title,
    todo::Todo,
};
use crate::usecase::todo::Usecase;
use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{header, StatusCode},
    web, HttpResponse,
};

extern crate derive_more;
use derive_more::Display;

#[derive(Display, Debug)]
enum TodoError {
    #[display(fmt = "invalid due_to format: {}", message)]
    ParseError { message: String },
    #[display(fmt = "internal error occured")]
    InternalError,
}

impl error::ResponseError for TodoError {
    fn status_code(&self) -> StatusCode {
        match *self {
            TodoError::ParseError { .. } => StatusCode::BAD_REQUEST,
            TodoError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/json")
            .json(self.to_string())
    }
}

pub struct Controller<T: Repository> {
    pub usecase: Usecase<T>,
}

pub struct CreateTodoReqest {
    pub title: String,
    pub due_to: String,
}

impl CreateTodoReqest {
    fn to_model(&self, id: Id) -> Option<Todo> {
        let title = Title::new(&self.title);
        let date = chrono::NaiveDate::parse_from_str(&self.due_to, "%Y/%m/%d").ok()?;
        let due_to = DueTo::new(&date);
        Some(Todo::new(&id, &title, &due_to))
    }
}

impl<T: Repository> Controller<T> {
    pub async fn create(
        &self,
        req: web::Json<CreateTodoReqest>,
    ) -> actix_web::Result<HttpResponse> {
        let id = generate();
        let todo = req.to_model(id).ok_or(TodoError::ParseError {
            message: "invalid format".to_string(),
        })?;

        let res = match self.usecase.create(&todo) {
            Ok(()) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Err(TodoError::InternalError),
        }?;
        Ok(res)
    }
}
