CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Basic info
    org_name            VARCHAR(150) NOT NULL,

    -- Auth
    password_hash   TEXT NOT NULL,
    email_verified  BOOLEAN DEFAULT FALSE,
    phone_verified  BOOLEAN DEFAULT FALSE,


    short_name      VARCHAR(50),
    domain          VARCHAR(100),        -- school.edu (optional login mapping)
    logo_url        TEXT,

    -- Contact info
    contact_email   VARCHAR(255) NOT NULL,
    contact_phone   VARCHAR(30) NOT NULL,
    org_address        TEXT,


    -- Subscription & tier
    plan            VARCHAR(50),   -- free, basic, pro, enterprise
    max_students    INTEGER,
    max_exams       INTEGER,

    -- Status
    is_active       BOOLEAN DEFAULT TRUE,
    is_suspended    BOOLEAN DEFAULT FALSE,

    -- Billing lifecycle
    subscription_start TIMESTAMP,
    subscription_end   TIMESTAMP,
    auto_renew         BOOLEAN DEFAULT TRUE,

    -- Auditing
    created_at      TIMESTAMP DEFAULT now(),
    updated_at      TIMESTAMP DEFAULT now()
);
