CREATE TABLE accounts (
    id bigserial primary key,
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

CREATE INDEX accounts_user_id_idx ON accounts(user_id);
CREATE INDEX accounts_provider_account_id_idx ON accounts(provider_account_id);

SELECT add_timestamps_trigger('accounts'::regclass);
