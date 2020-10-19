use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, Bson, document},
};
use serde_json::{Result, Value};

#[derive(Deserialize,Debug,Clone)]
pub struct Account{
    userName:Option<String>,
    password:Option<String>,
    token:Option<String>
}

impl Account{

    pub fn to_document(&mut self) ->document::Document{
        let mut doc = document::Document::new();

        doc.insert("userName",Bson::from(self.userName.take().unwrap()));
        doc.insert("password", Bson::from(self.password.take().unwrap()));
        doc.insert("token", Bson::from(self.token.take().unwrap_or("".parse().unwrap())));

        doc
    }
}