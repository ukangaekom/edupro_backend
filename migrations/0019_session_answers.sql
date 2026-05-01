CREATE TABLE IF NOT EXISTS session_answers(
     id BIGSERIAL PRIMARY KEY,
     user_id UUID REFERENCES users(id),
     session_id BIGINT REFERENCES exam_sessions(id),
     question_id BIGINT REFERENCES questions(id),

     question_answer INT DEFAULT 0,

     correct BOOLEAN,

     answered_at TIMESTAMPTZ DEFAULT now(),

     UNIQUE(session_id, question_id, user_id)
)