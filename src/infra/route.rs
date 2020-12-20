use std::error::Error;

use actix_web::{App, HttpServer, dev::Server, web};

use crate::{domain::repository::Repository, interface::controller::{self, todo::{Controller, CreateTodoReqest}}, usecase::todo::Usecase};

use super::db::mock;

async fn initialize() -> Result<(), std::io::Error> {
    let repository = mock::Db;
    let usecase = Usecase{repository};
    let controller = controller::todo::Controller{usecase};

    HttpServer::new(|| {
        App::new().service(
            web::resource("/todo")
                .data(web::JsonConfig::default().limit(1024))
                .route(web::post().to::<_>(|r: web::Json<CreateTodoReqest>| &controller.create(r))),
        )
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
