CREATE TABLE users (
    id bigserial primary key,
    email text NOT NULL UNIQUE COLLATE case_insensitive,
    email_verified timestamptz,
    image text,
    is_admin boolean NOT NULL DEFAULT false,
    last_active_at timestamptz NOT NULL DEFAULT now()
);

SELECT add_timestamps_trigger('users'::regclass);
