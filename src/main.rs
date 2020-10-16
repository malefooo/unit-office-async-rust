mod config;
mod util;
mod error;

use actix_web::{get, web, App, HttpServer, Responder};
use once_cell::sync::OnceCell;
use log::info;
use mongodb::Client;


#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

static CONF_INSTANCE: OnceCell<config::Config> = OnceCell::new();
static MONGODB_INSTANCE: OnceCell<Client> = OnceCell::new();

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    match CONF_INSTANCE.set(config::parsing_conf_toml()) {
        Ok(v)=>info!("config:{:?}",CONF_INSTANCE.get().unwrap()),
        Err(e)=>panic!("CONF_INSTANCE set err:{:?}",e)
    }

    let client = match config::init_mongodb().await {
        Ok(v) => v,
        Err(e)=>{panic!("panic")}
    };

    
    match MONGODB_INSTANCE.set(client) {
        Ok(v)=>info!("mongodb init success"),
        Err(e)=>panic!("MONGODB_INSTANCE set err:{:?}",e)
    }

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:5859")?
        .run()
        .await
}
