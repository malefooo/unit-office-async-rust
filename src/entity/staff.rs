use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, Bson, document},
};
use serde_json::value::Value::Null;

#[derive(Deserialize,Debug,Clone)]
pub struct Staff{
    fullName:Option<String>,
    mobile:Option<String>,
    email:Option<String>,
    position:Option<String>,
    description:Option<String>,
    photo:Option<String>
}

impl Staff{


    pub fn to_document(&mut self) -> document::Document{
        let mut doc = document::Document::new();

        doc.insert("fullName",Bson::from(self.fullName.take().unwrap_or("".parse().unwrap())));
        doc.insert("mobile", Bson::from(self.mobile.take().unwrap_or("".parse().unwrap())));
        doc.insert("email", Bson::from(self.email.take().unwrap_or("".parse().unwrap())));
        doc.insert("position", Bson::from(self.position.take().unwrap_or("".parse().unwrap())));
        doc.insert("description", Bson::from(self.description.take().unwrap_or("".parse().unwrap())));
        doc.insert("photo", Bson::from(self.photo.take().unwrap_or("".parse().unwrap())));

        doc
    }
}