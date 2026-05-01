CREATE TABLE IF NOT EXISTS user_xp_data(
    id BIGSERIAL PRIMARY KEY,

    session_id BIGINT REFERENCES exam_sessions(id),


    user_id UUID NULL References users(id),

    
    total_xp INT NOT NULL,

    UNIQUE(session_id, user_id)
)