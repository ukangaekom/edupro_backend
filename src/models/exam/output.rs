use serde::{Deserialize,Serialize};






#[derive(Serialize, Deserialize)]
pub struct options{
    pub option_a: String,
    pub option_b: String,
    pub option_c: String,
    pub option_d: String
}


#[derive(Serialize, Deserialize)]
pub struct selected_option{
    pub question_id: i32,
    pub option: i32
    
}


#[derive(Serialize, Deserialize)]
pub struct question{
    pub question_id: i32,
    pub question_image: Option<String>,
    pub question_options: options
}

#[derive(Serialize, Deserialize)]
pub struct exam_questions{
    pub exam_id: i64,
    pub question: Vec<question>

}



#[derive(Serialize, Deserialize)]
pub struct exam_details{
    pub exam_id: i64,
    pub exam_name: String,
    pub start_date: String,
    pub end_date: String,
}


#[derive(Serialize, Deserialize)]
pub struct exams{
    pub session_id: i64,
    pub exams: Vec<exam_details>
}
