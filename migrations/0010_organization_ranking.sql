CREATE TABLE IF NOT EXISTS organization_rank (

    organization_id UUID PRIMARY KEY REFERENCES organizations(id) ON DELETE CASCADE,

    total_students INT DEFAULT 0,

    total_exams INT DEFAULT 0


)