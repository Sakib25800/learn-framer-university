DROP INDEX IF EXISTS verification_tokens_token_idx;
DROP INDEX IF EXISTS verification_tokens_expires_idx;
DROP TABLE IF EXISTS verification_tokens;
DROP FUNCTION IF EXISTS generate_verification_token;
