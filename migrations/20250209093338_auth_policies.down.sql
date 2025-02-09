-- Add down migration script here
alter table "auth"."basic_user" disable row level security;