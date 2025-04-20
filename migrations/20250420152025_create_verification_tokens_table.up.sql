CREATE OR REPLACE FUNCTION generate_verification_token()
RETURNS text AS $$
BEGIN
    RETURN encode(gen_random_bytes(32), 'hex');
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS verification_tokens (
    identifier TEXT NOT NULL,
    token TEXT NOT NULL DEFAULT generate_verification_token(),
    expires TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (identifier, token)
);

CREATE UNIQUE INDEX IF NOT EXISTS verification_tokens_token_idx ON verification_tokens(token);
SELECT create_timestamp_triggers('verification_tokens');
