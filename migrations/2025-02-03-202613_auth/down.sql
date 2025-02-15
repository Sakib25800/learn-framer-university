-- First drop triggers
DROP TRIGGER IF EXISTS set_updated_at ON users;
DROP TRIGGER IF EXISTS set_updated_at ON accounts;

-- Then drop indexes
DROP INDEX IF EXISTS accounts_user_id_idx;
DROP INDEX IF EXISTS accounts_provider_account_id_idx;
DROP INDEX IF EXISTS refresh_tokens_token_idx;
DROP INDEX IF EXISTS refresh_tokens_user_id_idx;
DROP INDEX IF EXISTS users_last_active_at_idx;
DROP INDEX IF EXISTS verification_token_identifier_idx;

-- Then drop tables (in correct order due to dependencies)
DROP TABLE IF EXISTS refresh_tokens;
DROP TABLE IF EXISTS verification_tokens;
DROP TABLE IF EXISTS accounts;
DROP TABLE IF EXISTS users;

-- Then drop custom types and functions
DROP TYPE IF EXISTS auth_method_enum CASCADE;
DROP FUNCTION IF EXISTS generate_verification_token();
DROP COLLATION IF EXISTS case_insensitive;
