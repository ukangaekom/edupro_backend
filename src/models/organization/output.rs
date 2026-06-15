use serde::{Deserialize,Serialize};
use crate::models::exam::output::*;




#[derive(Serialize, Deserialize)]
pub struct organization_account_details{
    pub account_id: String,
    pub organization_name: String,
    pub contact_email: String,
    pub contact_phone: String,

}





#[derive(Serialize, Deserialize)]
pub struct organziation_exams{
    pub total_exams: i32,
    pub upcoming_exams: i32,
    pub past_exams: i32, 

}



#[derive(Serialize, Deserialize)]
pub struct organization_question{
    pub question_id: String,
    pub question_image: Option<String>,
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_option: i32,
}



#[derive(Serialize, Deserialize)]
pub struct organzation_question_bank{
    pub total_questions: i32,
    pub page: i32,
    pub per_page: i32,
    pub questions: Vec<Question>,
}


#[derive(Serialize, Deserialize)]
pub struct organization_students{
    pub total_students: i32,
    pub students: Vec<String>,

}



#[derive(Serialize, Deserialize)]
pub struct organization_score{
    pub position: i32,
    pub name: String,
    pub score: i32,
    
}



#[derive(Serialize, Deserialize)]
pub struct organization_leaderboard{
    pub leaderboard: Vec<organization_score>,
}

