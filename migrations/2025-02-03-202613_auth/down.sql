-- Drop triggers
DROP TRIGGER IF EXISTS set_updated_at ON users;

DROP TRIGGER IF EXISTS set_updated_at ON accounts;

-- Drop indexes
DROP INDEX IF EXISTS users_last_active_at_idx;

DROP INDEX IF EXISTS refresh_tokens_user_id_idx;

DROP INDEX IF EXISTS refresh_tokens_token_idx;

DROP INDEX IF EXISTS verification_token_idx;

DROP INDEX IF EXISTS accounts_provider_account_id_idx;

DROP INDEX IF EXISTS accounts_user_id_idx;

-- Drop tables
DROP TABLE IF EXISTS refresh_tokens;

DROP TABLE IF EXISTS verification_tokens;

DROP TABLE IF EXISTS accounts;

DROP TABLE IF EXISTS users;

-- Drop custom types and functions
DROP TYPE IF EXISTS auth_method_enum;

DROP FUNCTION IF EXISTS generate_verification_token ();

DROP COLLATION IF EXISTS case_insensitive;

-- Drop extension (optional, comment out if other parts of your app use it)
-- DROP EXTENSION IF EXISTS pgcrypto;
