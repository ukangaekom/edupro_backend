use serde::{Deserialize,Serialize};
use uuid::Uuid;




// Options




// Question

#[derive(Serialize, Deserialize)]
pub struct answer_question{
    pub id: i32
}


//Exams

#[derive(Serialize, Deserialize)]
pub struct set_exams{
    pub organization_id: Uuid,
    pub exam_name: String,
    pub start_date: String,
    pub end_date: String,
    pub total_subjects: i32,
    pub question_each: i32,

}


#[derive(Serialize, Deserialize)]
pub struct register_exam_payload{
    pub user_id: Uuid,
    pub exam_id: i32,
    pub subjects: String
}


#[derive(Serialize, Deserialize)]
pub struct start_exam_payload{
    pub user_id: Uuid,
    pub exams_id: i32,
}


#[derive(Serialize, Deserialize)]
pub struct practice_exam_payload{
    pub user_id: Uuid,
    pub practice_id: i32,
    pub subjects: String,
}



#[derive(Serialize, Deserialize)]
pub struct submit_exams{
    pub session_id: i64
}








