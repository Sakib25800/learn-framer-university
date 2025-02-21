CREATE TYPE auth_method_enum AS ENUM ('oauth', 'email');

CREATE OR REPLACE FUNCTION generate_verification_token()
RETURNS text AS $$
BEGIN
    RETURN encode(gen_random_bytes(32), 'hex');
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION generate_refresh_token()
RETURNS text AS $$
BEGIN
    RETURN encode(gen_random_bytes(32), 'hex');
END;
$$ LANGUAGE plpgsql;
