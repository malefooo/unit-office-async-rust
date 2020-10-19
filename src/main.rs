mod config;
mod util;
mod error;
mod server;
mod service;
mod entity;
mod constant;

use actix_web::{get, web, App, HttpServer, Responder};
use once_cell::sync::OnceCell;
use log::info;
use mongodb::Client;



static CONF_INSTANCE: OnceCell<config::Config> = OnceCell::new();
static MONGODB_INSTANCE: OnceCell<Client> = OnceCell::new();

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    config::init_log();

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

    //这里不加await的话，最后是一个future
    //因为这是一个async的方法，必须要加上一个await，让这个future进入到调度器中，去被调度，也就是被执行
    //我认为async和go的gpm有点像，但是底层实现是什么我还没有去深入
    server::http::init().await
}
