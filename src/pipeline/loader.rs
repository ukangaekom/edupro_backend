use glob::glob;
use sqlx::{PgPool, Row};
use serde::Deserialize;
use std::error::Error;
use std::collections::HashMap;


// CSV Format
#[derive(Debug, Deserialize)]
struct Questions_Records{

    #[serde(rename="Year")]
    year: i32,

    #[serde(rename="Question")]
    question: String,

    #[serde(rename="Option A")]
    option_a: String,

    #[serde(rename="Option B")]
    option_b: String,

    #[serde(rename="Option C")]
    option_c: String,

    #[serde(rename="Option D")]
    option_d: String,

    #[serde(rename="Answer")]
    answer: String,

    #[serde(rename="subject")]
    subject: String,

    #[serde(rename="exam_type")]
    exam_type: String,
}


pub async fn load_question_csv_to_db(conn:&mut PgPool) -> Result<(), Box<dyn Error>> {

    println!("Starting data ingestion pipeline...");

    // --- HERE ARE THE DECLARATIONS ---
    let mut type_cache: HashMap<String, i32> = HashMap::new();
    let mut year_cache: HashMap<String, i32> = HashMap::new();
    let mut subject_cache: HashMap<String,i32> = HashMap::new();
    // ---------------------------------

    // Initializing Large Transactions

    // Question Pipeline
    let pattern = "questions/*/*.csv";
    let mut total_inserted = 0;


    for entry in glob(pattern).expect("Failed to read glob pattern"){
        match entry{

            Ok(path)=>{

                println!("Reading file: {:?}", path);

                let mut reader = csv::Reader::from_path(&path)?;

                let mut file_count = 0;

                for result in reader.deserialize(){

                    let record: Questions_Records = match result{
                        Ok(rec) => {
                             println!("Successfully parsed record: {:?}", rec); rec},
                        Err(e) => {
                            eprintln!("  -> Skipping malformed row in {:?}: {}", path, e);
                            continue;
                        }
                    };

                    let mut exam_type_id = get_or_insert_exam_type(conn, &mut type_cache, &record.exam_type).await?;

                    println!("Exam Type ID for '{}': {}", record.exam_type, exam_type_id);

                    let mut exam_year_id = get_or_insert_exam_year(conn, &mut year_cache, &record.year).await?;
                    println!("Exam Year ID for '{}': {}", record.year, exam_year_id);
                    let mut subject_id = get_or_insert_subject(conn, &mut subject_cache, &record.subject).await?;
                    println!("Subject ID for '{}': {}", record.subject, subject_id);
                    let mut question_id = insert_questions(conn, subject_id, exam_year_id, exam_type_id, record.question, record.answer).await?;
                    println!("Inserted question with ID: {}", question_id);
                    let mut option_id = insert_options(conn, question_id, record.option_a, record.option_b, record.option_c, record.option_d).await?;
                    println!("Inserted Option with ID: {}", option_id);
                    
                    file_count += 1;
                    total_inserted += 1;
                    println!("{}",file_count);
                    println!("{}",total_inserted);
                }

                println!("  -> Inserted {} records from this file.", file_count);
            },

            Err(e) => eprintln!("Glob error: {:?}", e)
        
        }
        
    }

    // Create a connection pool to the PostgreSQL database
    Ok(())

}






async fn get_or_insert_subject(
    pool: &PgPool,
    cache: &mut HashMap<String, i32>,
    subject: &str,
)-> Result<i32, sqlx::Error>{

    if let Some(&id) = cache.get(subject) {
        return Ok(id);
    }

    let row = sqlx::query(
        r#"
        INSERT INTO subjects (subject_name) VALUES ($1)
        ON CONFLICT (subject_name) DO UPDATE SET subject_name = EXCLUDED.subject_name
        RETURNING id
        "#
    )
    .bind(subject)
    .fetch_one(pool)
    .await?;

    let id: i32 = row.get("id");

    cache.insert(subject.to_string(), id);
    Ok(id)



}



async fn get_or_insert_exam_type(
    pool: &PgPool,
    cache: &mut HashMap<String, i32>,
    exam_type: &str,
)-> Result<i32, sqlx::Error>{

    if let Some(&id) = cache.get(exam_type) {
        return Ok(id);
    }

    let row = sqlx::query(
        r#"
        INSERT INTO exam_type (exam_name) VALUES ($1)
        ON CONFLICT (exam_name) DO UPDATE SET exam_name = EXCLUDED.exam_name
        RETURNING id
        "#
    )
    .bind(exam_type)
    .fetch_one(pool)
    .await?;

    let id: i32 = row.get("id");

    cache.insert(exam_type.to_string(), id);
    Ok(id)



}



async fn get_or_insert_exam_year(
    pool: &PgPool,
    cache: &mut HashMap<String, i32>,
    exam_year: &i32,
)-> Result<i32, sqlx::Error>{

    if let Some(&id) = cache.get(&exam_year.to_string()) {
        return Ok(id);
    }

    let row = sqlx::query(
        r#"
        INSERT INTO exam_year (exam_year) VALUES ($1)
        ON CONFLICT (exam_year) DO UPDATE SET exam_year = EXCLUDED.exam_year
        RETURNING id
        "#
    )
    .bind(exam_year)
    .fetch_one(pool)
    .await?;

    let id: i32 = row.get("id");

    cache.insert(exam_year.to_string(), id);
    Ok(id)

}



async fn insert_questions(
    pool: &PgPool,
    subject_name: i32,
    exam_year: i32,
    exam_type: i32,
    question: String,
    correct_option: String
)->Result<i64, sqlx::Error>{

    let mut correct_answer = convert_letter_to_index(&correct_option).unwrap_or_default();
    println!("{}", correct_answer);

    let row = sqlx::query(
        r#"
        INSERT INTO questions (subject_name, exam_year, exam_type, question, correct_option) VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#
    )
    .bind(subject_name)
    .bind(exam_year)
    .bind(exam_type)
    .bind(question)
    .bind(correct_answer)
    .fetch_one(pool)
    .await?;

    let id: i64 = row.get("id");

    Ok(id)

}


async fn insert_options(
    pool: &PgPool,
    exam_question_id: i64,
    option_a: String,
    option_b: String,
    option_c: String,
    option_d: String,

) -> Result<i32, sqlx::Error>{

     let row = sqlx::query(
        r#"
        INSERT INTO options (question_id, option_a, option_b, option_c, option_d) VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (question_id) DO UPDATE SET question_id = EXCLUDED.question_id
        RETURNING id
        "#
    )
    .bind(exam_question_id)
    .bind(option_a)
    .bind(option_b)
    .bind(option_c)
    .bind(option_d)
    .fetch_one(pool)
    .await?;


    let id = row.get("id");

    Ok(id)


}



fn convert_letter_to_index(letter: &str) -> Option<i32> {
    // .trim() handles cases where the CSV has trailing spaces like "A "
    match letter.trim().to_uppercase().as_str() {
        "A" => Some(1),
        "B" => Some(2),
        "C" => Some(3),
        "D" => Some(4),
        _ => None, // Safely catch malformed data
    }
}