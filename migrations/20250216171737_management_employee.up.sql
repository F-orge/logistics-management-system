-- Add up migration script here

create schema management;



-- reference: https://www.indeed.com/recruitment/c/info/employee-information-form
-- reference: https://in.indeed.com/career-advice/starting-new-job/employee-information-form
create type management.employee_marital_status as enum ('Single','Married','Divorced', 'Widowed', 'Separated');

create table management.employee
(
    id                   uuid primary key        default gen_random_uuid(),
    -- note:
    -- full name must be unique so no duplicate information will be processed
    full_name            varchar(255)   not null unique,
    tel_number           varchar(255)   null,
    mobile_number        varchar(255)   null,
    -- note:
    -- this will be the same in auth.basic_user. if auth_type is different use any email
    email                varchar(255)   not null unique check ( email ~ '^.+@.+\..+$' ),
    phil_nat_id          varchar(12)    not null,
    birth_date           date           not null check ( birth_date <= current_date - interval '18 years' ),
    special_interests    varchar(255)[] null,
    learning_institution varchar(255)[] not null,
    auth_user_id         uuid references auth.users (id),
    spouse_name          varchar(255)   null,
    spouse_employer      varchar(255)   null,
    created_at           timestamp      not null default current_timestamp,
    updated_at           timestamp      not null default current_timestamp
);

create table management.department
(
    id          uuid primary key      default gen_random_uuid(),
    name        varchar(255) not null,
    -- note:
    -- markdown compatible text format
    description text         null,
    created_at  timestamp    not null default current_timestamp
);

create table management.job_information
(
    id            uuid primary key      default gen_random_uuid(),
    title         varchar(255) not null,
    employee_id   uuid         not null references management.employee (id),
    department_id uuid         not null references management.department (id),
    supervisor_id uuid         not null references management.employee (id),
    work_location varchar(255) not null,
    start_date    date         not null,
    salary        money        not null,
    currency      varchar(3)   not null check ( length(currency) = 3 ),
    created_at    timestamp    not null default current_timestamp,
    updated_at    timestamp    not null default current_timestamp
);

create table management.emergency_information
(
    id                uuid primary key default gen_random_uuid(),
    employee_id       uuid           not null references management.employee (id),
    address           varchar(255),
    contact_number    varchar(255)   not null,
    relationship      varchar(255)   not null,
    health_conditions varchar(255)[] null
);

-- note:
-- personnel action notice (pan) a document that records and notifies changes to an employee's status, such as a promotion, termination, or salary change
create type management.pan_action_type as enum ('Hire', 'Promotion', 'Salary Adjustment', 'Termination', 'Leave', 'Transfer');
create type management.pan_action_status as enum ('Pending','Approved','Rejected');
create table management.personnel_action
(
    id             uuid primary key                      default gen_random_uuid(),
    employee_id    uuid references management.employee (id),
    action_type    management.pan_action_type   not null,
    old_value      text,
    new_value      text,
    effective_date date                         not null,
    status         management.pan_action_status not null default 'Pending',
    requested_by   uuid                         not null references management.employee (id),
    approved_by    uuid                         not null references management.employee (id),
    created_at     timestamp                    not null default current_timestamp,
    updated_at     timestamp                    not null default current_timestamp
)

-- triggers