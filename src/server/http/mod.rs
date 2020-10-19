use actix_web::{get, web, App, HttpServer, Responder};
use log::info;
use actix_web::dev::Server;
use crate::service::common;
use crate::CONF_INSTANCE;


pub async fn init() -> std::io::Result<()> {

    let config = CONF_INSTANCE.get().unwrap();
    let server = config.ip.clone().server;

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/v1/common/register").route(web::post().to(common::register)))
    })
        .bind(server)?
        .run()
        .await
}

