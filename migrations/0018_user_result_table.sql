CREATE TABLE IF NOT EXISTS user_results(
    id BIGSERIAL PRIMARY KEY,


    session_id BIGINT REFERENCES exam_sessions(id),
    subject_id INT NOT NULL REFERENCES subjects(id),

    
    total_points INT NOT NULL,
    total_question INT NOT NULL,

    UNIQUE(session_id, subject_id)
)