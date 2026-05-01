CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    institution_id     UUID NULL References organizations(id),


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
    grade_level     VARCHAR(50),
    department      VARCHAR(100),


     -- Billing lifecycle
    subscription_start TIMESTAMPTZ,
    subscription_end   TIMESTAMPTZ,
    auto_renew         BOOLEAN DEFAULT TRUE,

    -- Status
    is_active       BOOLEAN DEFAULT TRUE,
    is_suspended    BOOLEAN DEFAULT FALSE,

    -- Security & tracking
    last_login_at   TIMESTAMPTZ,
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);
