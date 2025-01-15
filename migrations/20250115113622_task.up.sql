-- Add up migration script here

create type "management"."task_status" as enum ('UNASSIGNED','IN_PROGRESS','REVIEW','DONE');

create table "management"."task" (
  id uuid primary key default gen_random_uuid(),
  title varchar not null,
  description text,
  status "management"."task_status" not null,
  issued_by_employee_id uuid not null,
  deadline date not null,
  foreign key (issued_by_employee_id) references "management"."employee"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

create table "management"."task_comment" (
  id uuid primary key default gen_random_uuid(),
  task_id uuid not null,
  employee_id uuid not null,
  comment text not null,
  foreign key (task_id) references "management"."task"(id),
  foreign key (employee_id) references "management"."employee"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

create table "management"."task_comment_file_attachment" (
  id uuid primary key default gen_random_uuid (),
  file_id uuid not null,
  task_comment_id uuid not null,
  foreign key (file_id) references "resources"."file"(id),
  foreign key (task_comment_id) references "management"."task_comment"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);