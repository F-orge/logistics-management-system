-- Add up migration script here
-- reference: https://www.postgresql.org/docs/current/user-manag.html
-- RBAC role based access control
-- NOTE: this will automatically change after the migration is complete
-- extensions
-- internal system roles
create extension pgcrypto;

create extension pgjwt;

create user developer
    with
    encrypted password 'developer@password' superuser;

create user migration
    with
    encrypted password 'migration@password' superuser;

create role web
    with
    login password 'web@password' createrole;

create role anon nologin;
