-- Add up migration script here
create schema "auth";
create extension pgcrypto;
create extension pgjwt;
create user nextjs nobypassrls;

grant usage on schema "auth" to nextjs;