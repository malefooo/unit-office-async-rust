use crate::entity::role::Role;

pub struct CompanyRole{
    pub _id:Option<String>,
    pub company:Option<String>,
    pub roles:Option<Vec<Role>>
}

impl CompanyRole{
    pub fn new()->CompanyRole{
        CompanyRole{
            _id: None,
            company: None,
            roles: None
        }
    }
}