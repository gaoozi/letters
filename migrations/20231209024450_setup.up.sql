-- Set up database features that quite often

-- Add extension for better uuid
create extension if not exists "uuid-ossp";

-- Create functions for update time in insert
create or replace function set_updated_at()
  returns trigger as
$$
begin
  NEW.updated_at = now();
  return NEW;
end
$$ language plpgsql;

-- This function create a trigger
-- before update, the trigger will auto execute set_updated_at function
create or replace function trigger_updated_at(tablename regclass)
  returns void as
$$
begin
  execute format('CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON %s
    FOR EACH ROW
    WHEN (OLD is distinct from NEW)
  EXECUTE FUNCTION set_updated_at();', tablename);
end
$$ language plpgsql;

-- This is a text collation that sorts text case-insensitively,
create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
