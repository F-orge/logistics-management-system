-- Add up migration script here
create table
  logistics.department (
    id uuid primary key default gen_random_uuid (),
    department_name varchar(255) unique not null,
    manager_id uuid NULL references logistics.users (id),
    created timestamp not null default current_timestamp,
    updated timestamp not null default current_timestamp
  );

CREATE TABLE
  logistics.position (
    id uuid primary key default gen_random_uuid (),
    position_name VARCHAR(255) UNIQUE not null,
    job_description TEXT not null,
    created timestamp not null default current_timestamp,
    updated timestamp not null default current_timestamp
  );

create type logistics.employee_status as enum('active', 'inactive');

create type logistics.employee_contract_type as enum('full-time', 'part-time');

CREATE TABLE
  logistics.employee (
    id uuid primary key default gen_random_uuid (),
    first_name varchar(100) not null,
    last_name varchar(100) not null,
    email varchar(255) unique not null,
    phone varchar(20) unique,
    address text,
    status logistics.employee_status not null default 'active',
    cotract_type logistics.employee_contract_type not null,
    hire_date date not null,
    department_id uuid not null references logistics.department (id),
    position_id uuid not null references logistics.position (id),
    supervisor_id uuid null references logistics.employee (id),
    created timestamp not null default current_timestamp,
    updated timestamp not null default current_timestamp
  );

CREATE TABLE
  logistics.task (
    id uuid primary key default gen_random_uuid (),
    task_name VARCHAR(255) not null,
    description TEXT not null,
    assigned_by uuid not null references logistics.employee (id),
    assigned_to uuid not null references logistics.employee (id),
    start_date DATE not null,
    due_date DATE not null check (due_date >= start_date),
    priority VARCHAR(10) not null check (priority IN ('Low', 'Medium', 'High')),
    status VARCHAR(20) not null check (status IN ('Pending', 'In Progress', 'Completed')),
    created timestamp not null default current_timestamp,
    updated timestamp not null default current_timestamp
  );

create table
  logistics.task_messages (
    id uuid primary key default gen_random_uuid (),
    message text not null,
    task_id uuid not null references logistics.task (id),
    sender_id uuid not null references logistics.employee (id),
    created timestamp not null default current_timestamp,
    updated timestamp not null default current_timestamp
  )