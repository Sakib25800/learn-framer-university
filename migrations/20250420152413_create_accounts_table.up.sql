CREATE TYPE auth_method_enum AS ENUM ('email', 'oauth');

CREATE TABLE IF NOT EXISTS accounts (
    id bigserial PRIMARY KEY,
    user_id bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    auth_method auth_method_enum NOT NULL,
    provider text NOT NULL,
    provider_account_id text NOT NULL,
    refresh_token text,
    access_token text,
    expires_at bigint,
    token_type text,
    scope text,
    id_token text,
    UNIQUE(provider, provider_account_id)
);

CREATE INDEX IF NOT EXISTS accounts_user_id_idx ON accounts(user_id);
SELECT create_timestamp_triggers('accounts');
