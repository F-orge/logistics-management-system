-- Add up migration script here

create table "etmar_logistics"."task" (
  id uuid primary key default gen_random_uuid(),
  title varchar not null,
  description text,
  status varchar not null check (status in ( 'UNASSIGNED', 'IN_PROGRESS', 'REVIEW', 'DONE' )),
  issued_by_employee_id uuid not null,
  deadline date not null,
  foreign key (issued_by_employee_id) references "etmar_logistics"."employee"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

create table "etmar_logistics"."task_comment" (
  id uuid primary key default gen_random_uuid(),
  task_id uuid not null,
  employee_id uuid not null,
  comment text not null,
  foreign key (task_id) references "etmar_logistics"."task"(id),
  foreign key (employee_id) references "etmar_logistics"."employee"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

create table "etmar_logistics"."task_comment_file_attachment" (
  id uuid primary key default gen_random_uuid (),
  file_id uuid not null,
  task_comment_id uuid not null,
  foreign key (file_id) references "etmar_logistics"."file"(id),
  foreign key (task_comment_id) references "etmar_logistics"."task_comment"(id),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);