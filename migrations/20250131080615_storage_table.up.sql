-- Add up migration script here
create table "storage"."file" (
  id uuid primary key default gen_random_uuid(),
  name varchar not null,
  type varchar not null,
  size int not null,
  owner_id uuid,
  is_public boolean not null default false,
  foreign key (owner_id) references "auth"."user"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

-- NOTE: this is where we can do file sharing to other users
create table "storage"."file_access" (
  id uuid primary key default gen_random_uuid(),
  file_id uuid not null,
  user_id uuid not null,
  foreign key (file_id) references "storage"."file"(id),
  foreign key (user_id) references "auth"."user"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
)