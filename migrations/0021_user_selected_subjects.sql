CREATE TABLE IF NOT EXISTS select_exam_subjects(
    scheduled_exam_id BIGINT NOT NULL REFERENCES scheduled_exams(id),
    user_id UUID NOT NULL REFERENCES users(id),
    subject_id INT NOT NULL  REFERENCES subjects(id),
    question INT NOT NULL DEFAULT 50,


    PRIMARY KEY(scheduled_exam_id, user_id, subject_id)
)