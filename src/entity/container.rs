use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug,Clone)]
pub struct Container{
    pub key:Option<String>,
    pub code:Option<Value>,
    pub options:Option<Value>,
    pub containers:Option<Box<Container>>
}

