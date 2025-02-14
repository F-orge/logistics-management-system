-- Add up migration script here

create type "management"."role" as enum ('SUPER_ADMIN','MANAGER','EMPLOYEE');

create table "management"."employee" (
  id uuid primary key default gen_random_uuid(),
  user_id uuid null, -- set this to null for auto generation
  first_name varchar not null,
  last_name varchar not null,
  middle_name varchar not null,
  full_name varchar generated always as (first_name || ' ' || middle_name || ' ' || last_name) stored,
  role "management"."role" not null,
  address varchar not null,
  position varchar not null,
  birth_date date not null,
  avatar_file_id uuid null,
  cover_photo_file_id uuid null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  foreign key (user_id) references "auth"."user"(id),
  foreign key (avatar_file_id) references "storage"."file"(id),
  foreign key (cover_photo_file_id) references "storage"."file"(id)
);

grant all on "management"."employee" to nextjs;