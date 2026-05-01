use serde::{Deserialize,Serialize};




// User Registration 
#[derive(Debug, Deserialize)]
pub struct UserRegisterPayload{
    pub firstname:String,
    pub lastname: String,
    pub email:String,
    pub username: String,
    pub pwd: String,
}




// User Login
#[derive(Debug, Deserialize)]
pub struct LoginPayload{
    pub email: String,
    pub pwd: String,
}





