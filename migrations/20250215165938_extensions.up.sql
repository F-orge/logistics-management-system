-- Add up migration script here
-- reference: https://www.postgresql.org/docs/current/user-manag.html
-- RBAC role based access control
-- NOTE: this will automatically change after the migration is complete
-- extensions
-- internal system roles
create extension pgcrypto;

create extension pgjwt;