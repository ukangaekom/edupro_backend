CREATE TABLE IF NOT EXISTS exam_type(
    id SERIAL PRIMARY KEY,
    exam_name TEXT UNIQUE NOT NULL
)




-- INSERT INTO exam_type (id, exam_name) VALUES (1, 'Jamb'), (2, 'Waec'), (3, 'Neco') ON CONFLICT (id) DO NOTHING;