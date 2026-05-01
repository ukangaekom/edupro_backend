CREATE TABLE IF NOT EXISTS images (
    id BIGSERIAL PRIMARY KEY,
    image_url TEXT NOT NULL,           -- link to cloud storage
    file_name TEXT,
    mime_type TEXT,
    image_size BIGINT,
    created_at TIMESTAMPTZ DEFAULT now()
);