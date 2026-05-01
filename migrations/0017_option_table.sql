CREATE TABLE IF NOT EXISTS options(

    id SERIAL PRIMARY KEY,

    question_id INT REFERENCES questions(id),


     -- Options (fixed structure)
    option_a    TEXT NOT NULL, -- 1
    option_b    TEXT NOT NULL, -- 2
    option_c    TEXT NOT NULL, -- 3
    option_d    TEXT NOT NULL, -- 4

    UNIQUE(question_id)

)