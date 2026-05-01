CREATE TABLE IF NOT EXISTS questions (
    id  BIGSERIAL PRIMARY KEY,

    organization_id UUID NULL References organizations(id),

    subject_name    INT REFERENCES subjects(id),
    exam_year   INT NULL REFERENCES exam_year(id),
    exam_type   INT NULL REFERENCES exam_type(id),

    topic INT NULL REFERENCES topics(id),

    image_url   INT NULL REFERENCES images(id),  -- optional

    question    TEXT NOT NULL,

    correct_option INT NULL, -- 1. is option_a, 2. is option_b, 3. is option_c, 4. is option_d

    difficulty  INT NULL REFERENCES difficulty(id),

    customized BOOLEAN DEFAULT NULL,

    created_at  TIMESTAMPTZ DEFAULT now()

);