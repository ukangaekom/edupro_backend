use serde::{Deserialize,Serialize};


#[derive(Debug, Deserialize)]
pub struct LoginPayload{
    pub email: String,
    pub pwd: String,
}



#[derive(Debug, Deserialize)]
pub struct OrganizationRegisterPayload{
    pub organization:String,
    pub contact_email: String,
    pub contact_phone:String,
    pub pwd: String,
}


