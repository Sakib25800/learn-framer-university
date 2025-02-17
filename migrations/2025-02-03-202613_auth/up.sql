-- Create extension for cryptographic functions
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create case-insensitive collation
CREATE COLLATION IF NOT EXISTS case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);

-- Create function to generate verification tokens
CREATE OR REPLACE FUNCTION generate_verification_token()
RETURNS text AS $$
BEGIN
    RETURN encode(gen_random_bytes(32), 'hex');
END;
$$ LANGUAGE plpgsql;

-- Define ENUM for authentication method
CREATE TYPE auth_method_enum AS ENUM ('oauth', 'email', 'credentials', 'oidc', 'sms');

-- Core user data
CREATE TABLE users (
    id bigserial primary key,
    email text NOT NULL UNIQUE COLLATE case_insensitive,
    email_verified timestamptz,
    image text,
    is_admin boolean NOT NULL DEFAULT false,
    last_active_at timestamptz NOT NULL DEFAULT now(),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

-- OAuth provider accounts
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
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(provider, provider_account_id)
);

-- Verification tokens with user association
CREATE TABLE verification_tokens (
    identifier text NOT NULL,
    token text NOT NULL DEFAULT generate_verification_token(),
    expires timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY(identifier, token)
);

-- Refresh tokens for JWT authentication
CREATE TABLE refresh_tokens (
    id bigserial primary key,
    user_id bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token text NOT NULL UNIQUE,
    expires timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

-- Create indexes for common queries
CREATE INDEX accounts_user_id_idx ON accounts(user_id);
CREATE INDEX accounts_provider_account_id_idx ON accounts(provider_account_id);
CREATE INDEX refresh_tokens_token_idx ON refresh_tokens(token);
CREATE INDEX refresh_tokens_user_id_idx ON refresh_tokens(user_id);
CREATE INDEX users_last_active_at_idx ON users(last_active_at);
CREATE INDEX verification_token_identifier_idx ON verification_tokens(identifier);

-- Set up triggers for updated_at
SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('accounts');
