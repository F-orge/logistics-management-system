-- Add up migration script here
create schema "auth";

create extension pgcrypto;

create extension pgjwt;

do
$$
    begin
        if not exists (select 1 from pg_roles where rolname = 'web') then
            create role web;
        end if;
    end
$$;

grant usage on schema "auth" to web;