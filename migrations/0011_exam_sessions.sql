CREATE TABLE IF NOT EXISTS exam_sessions(
    id BIGSERIAL PRIMARY KEY,

    scheduled_exams_id INT NULL REFERENCES scheduled_exams(id),

    user_id UUID NOT NULL,

    session_type INT NULL REFERENCES exam_type(id), 

    mode INT NULL REFERENCES session_mode(id),

    total_question INT NOT NULL,

    subject_1 INT NOT NULL DEFAULT 0,

    subject_2 INT NOT NULL DEFAULT 0,

    subject_3 INT NOT NULL DEFAULT 0,

    subject_4 INT NOT NULL DEFAULT 0,

    start_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    expires_at TIMESTAMPTZ NOT NULL,

    session_status SMALLINT NOT NULL DEFAULT 0,


    CONSTRAINT unique_subject_check CHECK(
        (subject_1 = 0 OR 
        (subject_1 <> subject_2 AND subject_1 <> subject_3 AND subject_1 <> subject_4)) 
        AND (subject_2 = 0 OR (subject_2 <> subject_3 AND subject_2 <> subject_4 )) 
        AND (subject_3 = 0 OR (subject_3 <> subject_4))
        )

)