CREATE TABLE IF NOT EXISTS exam_year(
    id SERIAL PRIMARY KEY,
    exam_year INT UNIQUE NOT NULL
)





-- INSERT INTO exam_year (exam_year) SELECT generate_series(2000,2026) ON CONFLICT (exam_year) DO NOTHING;