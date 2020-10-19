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
    pub company:Option<Company>,
    pub roles:Option<Vec<Role>>,
}

pub fn document_to_customer(vo:document::Document){

    let mut customer = Customer{
        _id: None,
        account: None,
        status: None,
        staff: None,
        company: None,
        roles: None
    };

    let iter = vo.iter();
    for v in iter {
        
    }
}
