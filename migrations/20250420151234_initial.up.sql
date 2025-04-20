CREATE OR REPLACE FUNCTION set_created_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_timestamps_to_table(table_name regclass)
RETURNS void AS $$
BEGIN
    EXECUTE format('
        ALTER TABLE %I
        ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
    ', table_name);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION create_timestamp_triggers(table_name regclass)
RETURNS void AS $$
DECLARE
    trigger_name_created text;
    trigger_name_updated text;
BEGIN
    -- Generate unique trigger names
    trigger_name_created := table_name::text || '_set_created_at';
    trigger_name_updated := table_name::text || '_set_updated_at';

    -- Create created_at trigger
    EXECUTE format('
        CREATE TRIGGER %I
        BEFORE INSERT ON %I
        FOR EACH ROW
        EXECUTE FUNCTION set_created_at()
    ', trigger_name_created, table_name);

    -- Create updated_at trigger
    EXECUTE format('
        CREATE TRIGGER %I
        BEFORE UPDATE ON %I
        FOR EACH ROW
        EXECUTE FUNCTION set_updated_at()
    ', trigger_name_updated, table_name);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_all_timestamps(table_name regclass)
RETURNS void AS $$
BEGIN
    PERFORM add_timestamps_to_table(table_name);
    PERFORM create_timestamp_triggers(table_name);
END;
$$ LANGUAGE plpgsql;
