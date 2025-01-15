-- Add up migration script here

create table "management"."department"(
  id uuid primary key default gen_random_uuid(),
  name varchar not null,
  description text,
  manager_id uuid not null,
  foreign key (manager_id) references "management"."employee"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

alter table "management"."employee"
  add column department_id uuid;

alter table "management"."employee"
  alter column department_id set not null; 

alter table "management"."employee" 
  add constraint fk_department 
    foreign key (department_id) references "management"."department"(id);