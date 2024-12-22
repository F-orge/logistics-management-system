-- DO NOT EDIT. AUTO GENERATED --

create schema "auth"

create table "auth"."user" ("id" uuid default gen_random_uuid() not null primary key, "email" varchar(255) not null unique, "password" varchar(128) not null, "role" varchar(255) not null check (role IN ('super_admin', 'admin', 'employee', 'client')), "created_at" timestamp default current_timestamp not null, "updated_at" timestamp default current_timestamp not null)