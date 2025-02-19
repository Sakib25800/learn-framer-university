CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION add_timestamps_trigger(target_table regclass)
RETURNS VOID AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = target_table::text
        AND column_name = 'created_at'
    ) THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN created_at timestamptz NOT NULL DEFAULT now()', target_table::text);
    END IF;

    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = target_table::text
        AND column_name = 'updated_at'
    ) THEN
        EXECUTE format('ALTER TABLE %I ADD COLUMN updated_at timestamptz NOT NULL DEFAULT now()', target_table::text);
        PERFORM diesel_manage_updated_at(target_table);
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE COLLATION IF NOT EXISTS case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);
