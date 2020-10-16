use std::fs::File;
use std::io::Read;
use log::info;
use serde::{Deserialize, Serialize};
use mongodb::Client;
use crate::CONF_INSTANCE;
use std::error::Error;
use std::borrow::Borrow;
use mongodb::options::ClientOptions;
use crate::util;
use crate::error::CommonErrorEnum;

#[derive(Deserialize,Debug,Clone)]
pub struct Config{
    // pub server:Server,
    pub mongo:Mongo,
    pub mail:Mail,
    pub ip:Ip,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Server{
    pub port:i32,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Mongo{
    pub addr:String,
    pub database:String,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Mail{
    pub host:String,
    pub port:i32,
    pub username:String,
    pub password:String,
    pub encoding:String,
    pub protocol:String
}

#[derive(Deserialize,Debug,Clone)]
pub struct Ip{
    pub localhost:String,
    pub remote:String
}

pub fn parsing_conf_toml() -> Config{
    let file_path = "Conf.toml";
    let mut file = match File::open(file_path){
        Ok(f)=>f,
        Err(e)=>panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(_s)=>{},
        Err(e) => panic!("Error Reading file: {}", e)
    }

    toml::from_str(&str_val).unwrap()
}

pub async fn init_mongodb() -> Result<Client, CommonErrorEnum> {
    let mut op_config = CONF_INSTANCE.get().clone();
    let mut config = op_config.take().unwrap();
    let mut mongo = config.clone().mongo;
    let mut client_options = ClientOptions::parse(util::string_to_static_str(mongo.addr)).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}