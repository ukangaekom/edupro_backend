use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::env;
use std::option;
use uuid::Uuid;



pub static SECRET: Lazy<Vec<u8>> = Lazy::new(||{
    dotenvy::dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("Secret token must be set");

    secret.into_bytes()
});



// Defining the information for JWT
#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    sub: Uuid,
    exp: usize,
    role: String,

}

pub async fn create_jwt_token(login_id:&Uuid,role:&str) -> String {

    // Setting expiration date
    let exp = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .unwrap_or_default()
        .timestamp() as usize;

    let claims = Claims{
        sub: *login_id,
        exp,
        role: role.to_string(),
        
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(&SECRET)).unwrap()
    
}
pub async fn verify_jwt(token:&str)-> Option<Uuid>{

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&SECRET),
        &Validation::default(),
    )
    .ok()
    .map(|data| data.claims.sub)


} 

