use serde::{Deserialize,Serialize};
use sqlx::FromRow;

use crate::models::exam::output::*;






#[derive(Serialize, Deserialize,FromRow)]
pub struct UserAccountDetails{
    pub firstname: String, 
    pub lastname: String, 
    pub email: String,
    pub username: String,
    pub total_xps: i32,
    pub rank: i32,
    pub total_exams_taken: i32,
    pub total_practices_taken: i32,
    

}



#[derive(Serialize, Deserialize)]
pub struct user_registered_exams{
    pub exams: Vec<ExamDetails>,

}


// pub struct user_taken_exams{
//     pub exams: Vec<exam_details>,


// }

#[derive(Serialize, Deserialize)]
pub struct user_exam_scores{
    pub exams: Vec<ExamDetails>,
    pub scores: Vec<i32>,
    pub percents: Vec<f32>
}



#[derive(Serialize, Deserialize)]
pub struct user_practice_scores{
    pub practices: Vec<ExamDetails>,
    pub scores: Vec<i32>,
    pub percents: Vec<f32>

}



#[derive(Serialize, Deserialize)]
pub struct user_leaderboard{
    pub user_names: Vec<String>,
    pub users_xps: Vec<i32>,
    pub user_name: String,
    pub user_total_xp: i32,
    pub rank: i32,
}



#[derive(Serialize, Deserialize)]
pub struct user_exam_analytics{
    pub exam_id: i32,
    pub exam_name: String, 
    pub exam_date: String,
    pub total_score: i32,
    pub percent_score: f32,
    pub subject_analytics: Vec<subject_analytics>

}



#[derive(Serialize, Deserialize)]
pub struct subject_analytics{
    pub exam_id: i64,
    pub subject_id: i32,
    pub subject_name: String,
    pub score: i32,
    pub total_questions: i32,
    pub percent_score: f32,
}




