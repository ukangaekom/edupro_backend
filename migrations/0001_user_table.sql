CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Identity
    firstname      VARCHAR(100) NOT NULL,
    lastname       VARCHAR(100) NOT NULL,
    username        VARCHAR(50) UNIQUE,
    email           VARCHAR(255) UNIQUE NOT NULL,
    phone           VARCHAR(20),

    -- Auth
    password_hash   TEXT NOT NULL,
    email_verified  BOOLEAN DEFAULT FALSE,
    phone_verified  BOOLEAN DEFAULT FALSE,

    -- Role & access
    -- role            VARCHAR(30) NOT NULL,  -- student, teacher, admin, examiner

    -- Academic metadata
    student_id      VARCHAR(50),   -- matric / reg no
    institution     VARCHAR(150),
    grade_level     VARCHAR(50),
    department      VARCHAR(100),

    -- Status
    is_active       BOOLEAN DEFAULT TRUE,
    is_suspended    BOOLEAN DEFAULT FALSE,

    -- Security & tracking
    last_login_at   TIMESTAMP,
    created_at      TIMESTAMP DEFAULT now(),
    updated_at      TIMESTAMP DEFAULT now()
);
