use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug,Clone)]
pub struct Role{
    pub name:Option<String>,
    pub description:Option<String>,
    pub status:Option<i32>
}