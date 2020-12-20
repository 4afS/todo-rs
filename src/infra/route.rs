use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::{interface::controller, usecase::todo::Usecase};

use super::db::mock;

pub fn route(_: &mut web::ServiceConfig) {}

pub async fn run() -> Result<(), std::io::Error> {
    let repository = mock::Db;
    let usecase = Usecase { repository };
    let _ = controller::todo::Controller { usecase };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/todo").configure(route))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
