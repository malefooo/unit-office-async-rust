use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, Bson, document},
};

#[derive(Deserialize,Debug,Clone)]
pub struct Company {
    pub _id:Option<String>,
    pub name:Option<String>,
    pub description:Option<String>,
    pub address:Option<String>,
    pub phone:Option<String>,
    pub email:Option<String>,
    pub website:Option<String>,
    pub children:Option<Vec<String>>
}

impl Company {
    
    pub fn new() -> Company{
        Company{
            _id: None,
            name: None,
            description: None,
            address: None,
            phone: None,
            email: None,
            website: None,
            children: None
        }
    }

    pub fn to_document(&mut self) -> document::Document {
        let mut doc = document::Document::new();

        doc.insert("name",Bson::from(self.name.take().unwrap()));
        doc.insert("description", Bson::from(self.description.take().unwrap_or("".parse().unwrap())));
        doc.insert("address", Bson::from(self.address.take().unwrap_or("".parse().unwrap())));
        doc.insert("phone", Bson::from(self.phone.take().unwrap_or("".parse().unwrap())));
        doc.insert("email", Bson::from(self.email.take().unwrap_or("".parse().unwrap())));
        doc.insert("website", Bson::from(self.website.take().unwrap_or("".parse().unwrap())));
        doc.insert("children", Bson::from(self.children.take().unwrap_or(Vec::new())));

        doc
    }
}