CREATE TABLE verification_tokens (
    identifier text NOT NULL,
    token text NOT NULL DEFAULT generate_verification_token(),
    expires timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY(identifier, token)
);

CREATE TABLE refresh_tokens (
    id bigserial primary key,
    user_id bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token text NOT NULL UNIQUE DEFAULT generate_refresh_token(),
    expires timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX refresh_tokens_token_idx ON refresh_tokens(token);
CREATE INDEX refresh_tokens_user_id_idx ON refresh_tokens(user_id);
CREATE INDEX verification_token_identifier_idx ON verification_tokens(identifier);
