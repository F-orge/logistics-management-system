-- Add up migration script here
-- reference: https://www.postgresql.org/docs/current/user-manag.html
-- RBAC role based access control
-- NOTE: this will automatically change after the migration is complete
-- extensions
-- internal system roles
create extension pgcrypto;

create extension pgjwt;

do $$
begin
    if not exists (select 1 from pg_roles where rolname = 'developer') then
        create user developer
            with
            encrypted password 'developer@password' superuser;
    end if;
end
$$;

do $$
begin
    if not exists (select 1 from pg_roles where rolname = 'migration') then
        create user migration
            with
            encrypted password 'migration@password' superuser;
    end if;
end
$$;

do $$
begin
    if not exists (select 1 from pg_roles where rolname = 'web') then
        create role web
            with
            login password 'web@password' createrole;
    end if;
end
$$;

do $$
begin
    if not exists (select 1 from pg_roles where rolname = 'anon') then
        create role anon nologin;
    end if;
end
$$;