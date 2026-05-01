CREATE TABLE IF NOT EXISTS user_rank( 
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,

    total_xp INT DEFAULT 0,

    total_session INT DEFAULT 0,

    update_at TIMESTAMPTZ DEFAULT now()

)
