-- Add up migration script here

create table "etmar_logistics"."department"(
  id uuid primary key default gen_random_uuid(),
  name varchar not null,
  description text,
  manager_id uuid not null,
  foreign key (manager_id) references "etmar_logistics"."employee"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

alter table "etmar_logistics"."employee"
  add column department_id uuid;

alter table "etmar_logistics"."employee"
  alter column department_id set not null; 

alter table "etmar_logistics"."employee" 
  add constraint fk_department 
    foreign key (department_id) references "etmar_logistics"."department"(id);