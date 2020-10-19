use crate::entity::component::Component;

pub struct CompanyComponent{
    pub _id:Option<String>,
    pub company:Option<String>,
    pub components:Option<Vec<Component>>
}