CREATE TABLE IF NOT EXISTS scheduled_exams(
    id BIGSERIAL PRIMARY KEY,

    organization_id UUID NOT NULL REFERENCES organizations(id),

    exam_name VARCHAR(255) NOT NULL,

    exam_start_date TIMESTAMPTZ,

    exam_end_date TIMESTAMPTZ,

    create_at TIMESTAMPTZ DEFAULT now()
   
)