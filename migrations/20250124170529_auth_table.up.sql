-- Add up migration script here
create table "auth"."user" (
  id uuid primary key default gen_random_uuid(),
  auth_type varchar not null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

create table "auth"."basic_user" (
  email varchar primary key,
  password varchar not null,
  user_id uuid,
  foreign key (user_id) references "auth"."user"(id) on delete cascade
);

grant all on "auth"."user" to nextjs;
grant all on "auth"."basic_user" to nextjs;