use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug)]
pub struct Menu{
    name:Option<String>,
    size:Option<String>,
    ico:Option<String>,
    cursor:Option<String>,
    url:Option<String>,
}