use crate::entity::account::Account;
use crate::entity::staff::Staff;
use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, Bson, document},
};
use serde_json::{Result, Value};
use std::collections::HashMap;
use crate::entity::company::Company;
use serde::ser::Error;
use crate::entity::role::Role;

#[derive(Deserialize,Debug,Clone)]
pub struct Customer{
    pub _id:Option<String>,
    pub account:Option<Account>,
    pub status:Option<i32>,
    pub staff:Option<Staff>,
    pub company:Option<Value>,
    pub roles:Option<Vec<Role>>,
}

impl Customer {
    pub fn to_document(&mut self) -> document::Document{
        let mut doc = document::Document::new();


        doc.insert("account",Bson::from(self.account.take().unwrap().to_document()));
        doc.insert("status", Bson::from(self.status.take().unwrap()));
        doc.insert("staff", Bson::from(self.staff.take().unwrap().to_document()));
        doc.insert("roles",Bson::from(self.roles.take().unwrap()));
        let value = self.company.take().unwrap();
        if value.is_string() {
            let str = value.as_str().unwrap();
            doc.insert("company", Bson::from(str));
        } else {
            let mut company = serde_json::from_value::<Company>(value).unwrap();
            doc.insert("company", Bson::from(company.name.unwrap()));
        }


        doc
    }

}
