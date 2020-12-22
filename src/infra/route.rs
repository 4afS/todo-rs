use crate::infra::db::inmemory::Db;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::{application::todo, usecase::todo::Usecase};

extern crate env_logger;

pub fn route(config: &mut web::ServiceConfig) {
    config.route("/todo", web::post().to(todo::create::<Db>));
    config.route("/todo", web::get().to(todo::all::<Db>));
    config.route("/todo/{id}", web::delete().to(todo::delete::<Db>));
    config.route("/todo/{id}", web::get().to(todo::get::<Db>));
}

#[actix_web::main]
pub async fn run() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let repository = Db::new();
    let usecase = Usecase { repository };

    HttpServer::new(move || {
        App::new()
            .data(usecase.clone())
            .wrap(Logger::default())
            .service(web::scope("/").configure(route))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
