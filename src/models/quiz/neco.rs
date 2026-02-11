use crate::errors::error::{Error};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


// Jamb


#[derive(Clone,Debug, Serialize)]
pub struct Neco{
    pub year: String,
    
    pub subject: Vec<String>,
}



#[derive(Clone, Debug, Serialize)]
pub struct Neco_option{
    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
}


#[derive(Clone, Debug, Serialize)]
pub struct Neco_question{
    pub question: String,
    pub option: Neco_option
}


#[derive(Clone, Debug, Serialize)]
pub struct subject{
    pub name: String,

    pub question: u8,

    pub questions: Vec<Neco_question>,

}


