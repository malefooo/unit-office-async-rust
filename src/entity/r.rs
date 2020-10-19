
use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, Bson, document},
};
use serde_json::{Result, Value};
use crate::entity;

#[derive(Deserialize,Debug,Clone,Serialize)]
pub struct R{
    pub code:Option<i32>,
    pub message:Option<String>,
    pub data:Option<Value>
}

