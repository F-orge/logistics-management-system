-- Add up migration script here

create table "etmar_logistics"."file" (
  id uuid primary key default gen_random_uuid(),
  name varchar not null unique,
  path text not null,
  owner_id uuid not null,
  foreign key (owner_id) references "etmar_logistics"."user"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);