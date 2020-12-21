use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use mock::Db;

use crate::{interface::todo, usecase::todo::Usecase};

use super::db::mock;

extern crate env_logger;

pub fn route(config: &mut web::ServiceConfig) {
    let repository = Db;
    let usecase = Usecase { repository };

    config.data(usecase);
    config.route("/todo", web::post().to(todo::create::<Db>));
    config.route("/todo", web::get().to(todo::all::<Db>));
    config.route("/todo/{id}", web::delete().to(todo::delete::<Db>));
    config.route("/todo/{id}", web::get().to(todo::get::<Db>));
}

#[actix_web::main]
pub async fn run() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/").configure(route))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
