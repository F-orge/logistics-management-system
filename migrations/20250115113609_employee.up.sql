-- Add up migration script here

create table "etmar_logistics"."employee" (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null,
  first_name varchar not null,
  middle_name varchar,
  last_name varchar not null,
  sex varchar not null check (sex in ('MALE', 'FEMALE')),
  address text not null,
  position text not null,
  contact_number text not null,
  contract_type varchar not null check (contract_type in ('FULL_TIME', 'PART_TIME')),
  birth_day date not null,
  foreign key (user_id) references "etmar_logistics"."user" (id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);