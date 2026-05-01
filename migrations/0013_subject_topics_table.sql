CREATE TABLE IF NOT EXISTS topics(
    id SERIAL PRIMARY KEY,
    
    topic_name TEXT NOT NULL,

    subject_id INT REFERENCES subjects(id)

)