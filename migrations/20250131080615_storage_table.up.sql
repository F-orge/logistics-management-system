-- Add up migration script here
create table "storage"."file" (
  id uuid primary key default gen_random_uuid(),
  name varchar not null,
  type varchar not null,
  size int not null,
  owner_id uuid,
  foreign key (owner_id) references "auth"."user"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);