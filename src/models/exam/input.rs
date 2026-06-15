use serde::{Deserialize,Serialize};
use uuid::Uuid;




// Options




// Question

#[derive(Serialize, Deserialize)]
pub struct AnswerQuestion{
    pub id: i32
}


//Exams

#[derive(Serialize, Deserialize)]
pub struct SetExams{
    pub organization_id: Uuid,
    pub exam_name: String,
    pub start_date: String,
    pub end_date: String,
    pub total_subjects: i32,
    pub question_each: i32,

}



#[derive(Serialize, Deserialize)]
pub struct RegisterExamPayload{
    pub user_id: Uuid,
    pub exam_id: i32,
    pub subjects: String
}


#[derive(Serialize, Deserialize)]
pub struct StartExamPayload{
    pub user_id: Uuid,
    pub exams_id: i32,
}


#[derive(Serialize, Deserialize)]
pub struct PracticeExamPayload{
    pub user_id: Uuid,
    pub practice_id: i32,
    pub subjects: String,
}



#[derive(Serialize, Deserialize)]
pub struct SubmitExams{
    pub session_id: i64
}








