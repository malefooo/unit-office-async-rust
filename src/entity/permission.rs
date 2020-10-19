use crate::entity::menu::Menu;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug)]
pub struct Permission{
    subSystem:Option<String>,
    menus:Option<Vec<Menu>>
}