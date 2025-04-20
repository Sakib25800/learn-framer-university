CREATE TABLE IF NOT EXISTS users (
    id bigserial primary key,
    email text NOT NULL UNIQUE,
    email_verified timestamptz,
    image text,
    is_admin boolean NOT NULL DEFAULT false,
    last_active_at timestamptz NOT NULL DEFAULT now(),
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS users_email_idx ON users(email);

SELECT create_timestamp_triggers('users');
