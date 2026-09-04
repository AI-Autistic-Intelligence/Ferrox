-- Revoke default privileges on the public schema from PUBLIC
REVOKE CREATE ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON DATABASE yalc_db FROM PUBLIC;

-- Create an application-specific user with limited privileges
CREATE ROLE yalc_app WITH LOGIN PASSWORD 'secure_app_password';

-- Grant connect to the database
GRANT CONNECT ON DATABASE yalc_db TO yalc_app;

-- Grant usage on the public schema (assuming we use public, or a dedicated schema)
GRANT USAGE ON SCHEMA public TO yalc_app;

-- Grant CRUD permissions on all current tables, sequences, and functions
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO yalc_app;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO yalc_app;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO yalc_app;

-- Ensure future tables/sequences/functions get the same permissions automatically
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO yalc_app;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO yalc_app;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT EXECUTE ON FUNCTIONS TO yalc_app;

-- At this point, `yalc_app` can read/write data but CANNOT drop tables, create databases, or manage roles.
