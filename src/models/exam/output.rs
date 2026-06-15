use serde::{Deserialize,Serialize};
use sqlx::FromRow;





#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Options{
    pub option_a: String,
    pub option_b: String,
    pub option_c: String,
    pub option_d: String
}


#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct SelectedOption{
    pub question_id: i32,
    pub option: i32
    
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Question{
    pub question_id: i32,
    pub question_image: Option<String>,
    pub question_options: Options
}

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct ExamQuestions{
    pub exam_id: i64,
    pub question: Vec<Question>

}



#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct ExamDetails{
    pub exam_id: i64,
    pub exam_name: String,
    pub start_date: String,
    pub end_date: String,
}


#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct Exams{
    pub session_id: i64,
    pub exams: Vec<ExamDetails>
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExamType{
    pub id: i32,

    #[sqlx(rename = "exam_name")]
    pub name: String
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExamTypeSubject{
    pub id: i32,
    pub name: String,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>

}
