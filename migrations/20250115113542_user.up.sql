-- Add up migration script here

create schema auth;

create type "auth"."role" as enum ('SUPER_ADMIN','ADMIN','EMPLOYEE','CLIENT'); 

create table "auth"."user" (
  id uuid primary key default gen_random_uuid(),
  email varchar unique not null,
  password varchar unique not null,
  role varchar unique not null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);