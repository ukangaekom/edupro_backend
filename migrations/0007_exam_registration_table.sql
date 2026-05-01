CREATE TABLE IF NOT EXISTS exams_registration(
    id BIGSERIAL PRIMARY KEY,

    scheduled_exam_id BIGINT NOT NULL REFERENCES scheduled_exams(id),

    registered_student_id UUID NULL References users(id),

    registered_at TIMESTAMPTZ DEFAULT now(),


    UNIQUE(registered_student_id, scheduled_exam_id)


    

)