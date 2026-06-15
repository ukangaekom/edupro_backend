use sqlx::{PgPool, FromRow,Row,types::Uuid};
use crate::models::exam::output::*;



pub async fn get_exam_types(pool: &PgPool) -> Result<Vec<ExamType>, sqlx::Error> {
    let exam_types = sqlx::query_as::<_, ExamType>(
        r#"
        SELECT id, exam_name FROM exam_type
        "#
    ).fetch_all(pool).await?;
    
    
    return Ok(exam_types)  
}


pub async fn get_exam_type_subjects(pool: &PgPool) -> Result<Vec<ExamTypeSubject>, sqlx::Error>{

    // Select questions by exam type 
    let subjects = sqlx::query_as::<_, ExamTypeSubject>(
       r#"
        WITH year_stats AS (
            -- 1. Get the global min and max years
            SELECT 
                MIN(exam_year) AS start_year, 
                MAX(exam_year) AS end_year 
            FROM exam_year
            -- UNCOMMENT THIS if years belong to an exam_type:
            -- WHERE exam_type_id = $1 
        )
        -- 2. Attach those years to every subject
        SELECT 
            s.id, 
            s.subject_name AS name, 
            y.start_year, 
            y.end_year
        FROM subjects s
        CROSS JOIN year_stats y
        ORDER BY s.subject_name ASC
        "#).fetch_all(pool).await?;
    return Ok(subjects)
}

// fn get_exam_year(){
//     todo!()
// }


// fn get_exam_subjects(){
//     todo!()
// }


// fn get_exam_participants(){
//     todo!()
// }


// fn get_exam_creator(){
//     todo!()
// }


// fn get_exam_start_date(){

//     todo!()
// }


// fng get_exam_end_date(){
//     todo!()
// }


// fn get_exam_details(){
//     todo!()
// }


// fn get_exam_details(){
//     todo!()
// }


// fn get_exam_display(){
//     todo!()
// }


// fn get_subject(){
//     todo!()
// }


// fn get_subject_years(){
//     todo!()
// }

