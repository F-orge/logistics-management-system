-- Add down migration script here
drop schema "auth" cascade;
drop extension pgcrypto;