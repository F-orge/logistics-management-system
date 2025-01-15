-- Add up migration script here

create schema "etmar_logistics";

create table "etmar_logistics"."user" (
  id uuid primary key default gen_random_uuid(),
  email varchar unique not null,
  password varchar unique not null,
  user_role varchar not null check (user_role in ( 'SUPER_ADMIN', 'ADMIN', 'EMPLOYEE', 'CLIENT' )),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);