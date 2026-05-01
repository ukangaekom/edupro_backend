CREATE TABLE IF NOT EXISTS difficulty (
    id SERIAL PRIMARY KEY,
    
    difficulty_level TEXT UNIQUE NOT NULL
)