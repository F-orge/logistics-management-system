-- Add up migration script here

create schema "management";

create type "management"."sex" as enum ('MALE','FEMALE');
create type "management"."contract_type" as enum ('FULL_TIME','PART_TIME');

create table "management"."employee" (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null,
  first_name varchar not null,
  middle_name varchar,
  last_name varchar not null,
  sex "management"."sex" not null,
  address text not null,
  position text not null,
  contact_number text not null,
  contract_type "management"."contract_type" not null,
  birth_day date not null,
  foreign key (user_id) references "auth"."user" (id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);