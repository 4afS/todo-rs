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
    web::{self},
    HttpResponse,
};

extern crate derive_more;
use derive_more::Display;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct IdReqest {
    pub id: String,
}

impl IdReqest {
    fn to_model(&self) -> Option<Id> {
        Uuid::parse_str(&self.id).ok()
    }
}

trait ToResponse<T> {
    fn to_response(&self) -> Option<T>;
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub due_to: String,
}

impl TodoResponse {
    fn from_model(todo: &Todo) -> Self {
        let id = todo.id.to_string();
        let title = todo.title.value;
        let due_to = todo.due_to.date.format("%Y/%m/%d").to_string();
        TodoResponse { id, title, due_to }
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

    pub async fn delete(&self, req: web::Json<IdReqest>) -> actix_web::Result<HttpResponse> {
        let id = req.to_model().ok_or(TodoError::ParseError {
            message: "invalid format".to_string(),
        })?;

        let res = match self.usecase.remove(&id) {
            Ok(()) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Err(TodoError::InternalError),
        }?;
        Ok(res)
    }

    pub async fn get(&self, req: web::Json<IdReqest>) -> actix_web::Result<HttpResponse> {
        let id = req.to_model().ok_or(TodoError::ParseError {
            message: "invalid format".to_string(),
        })?;

        let res = match self.usecase.get(&id) {
            Ok(todo) => Ok(HttpResponse::Ok().json(TodoResponse::from_model(&todo))),
            Err(_) => Err(TodoError::InternalError),
        }?;
        Ok(res)
    }

    pub async fn all(&self) -> actix_web::Result<HttpResponse> {
        let res = match self.usecase.all() {
            Ok(todos) => Ok(HttpResponse::Ok().json(
                todos
                    .iter()
                    .map(|todo: &Todo| TodoResponse::from_model(todo))
                    .collect::<Vec<TodoResponse>>(),
            )),
            Err(_) => Err(TodoError::InternalError),
        }?;
        Ok(res)
    }
}
