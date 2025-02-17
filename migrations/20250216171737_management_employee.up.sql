-- Add up migration script here

create schema management;

-- reference: https://www.indeed.com/recruitment/c/info/employee-information-form
-- reference: https://in.indeed.com/career-advice/starting-new-job/employee-information-form
create type management.employee_marital_status as enum ('Single','Married','Divorced', 'Widowed', 'Separated');

create type management.employee_role as enum ('Admin','Manager','Employee');

create type management.employee_status as enum ('Active','Inactive');

create table management.employee
(
    id                   uuid primary key                    default gen_random_uuid(),
    -- profile picture
    avatar               storage.file               null,
    -- profile cover photo
    cover_photo          storage.file               null,
    -- note:
    -- full name must be unique so no duplicate information will be processed
    first_name           varchar(255)               not null,
    middle_name          varchar(255)               not null,
    last_name            varchar(255)               not null,
    constraint unique_employee_name unique (first_name, middle_name, last_name),
    -- references: https://en.wikipedia.org/wiki/Telephone_numbers_in_the_Philippines#:~:text=Mobile%20phone%20numbers%20are%20always,a%20seven%2Ddigit%20number).
    tel_number           varchar(10)                null,
    /*
        Mobile numbers in the Philippines are 10 or 12 digits long.
        Local mobile numbers
            Can be 11 digits long, such as 09171234567
            Can be 12 digits long, such as 639181234567
            Do not include special characters like + - . #
    */
    mobile_number        varchar(12)                null check (mobile_number ~ '^(09[0-9]{9}|639[0-9]{9})$'),
    -- note:
    -- this will be the same in auth.basic_user. if auth_type is different use any email
    email                varchar(255) unique check ( email ~ '^.+@.+\..+$' ),
    role                 management.employee_role   not null default 'Employee',
    status               management.employee_status not null default 'Active',
    -- note:
    -- philippine national id format is xxxx-xxxx-xxxx-xxxx
    phil_nat_id          varchar(19)                not null check (phil_nat_id ~ '^[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{4}$'),
    birth_date           date                       not null check ( birth_date <= current_date - interval '18 years' ),
    special_interests    varchar(255)[]             null,
    learning_institution varchar(255)[]             not null,
    auth_user_id         uuid references auth.users (id) on delete cascade,
    spouse_first_name    varchar(255)               null,
    spouse_middle_name   varchar(255)               null,
    spouse_last_name     varchar(255)               null,
    constraint unique_spouse_name unique (spouse_first_name, spouse_middle_name, spouse_last_name),
    spouse_employer      varchar(255)               null,
    created_at           timestamp                  not null default current_timestamp,
    updated_at           timestamp                  not null default current_timestamp
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

create table management.department_employees
(
    department_id uuid references management.department (id) on delete cascade,
    employee_id   uuid references management.employee (id) on delete cascade,
    primary key (department_id, employee_id)
);

create table management.job_information
(
    id            uuid primary key      default gen_random_uuid(),
    title         varchar(255) not null,
    employee_id   uuid         not null references management.employee (id),
    department_id uuid         not null references management.department (id),
    supervisor_id uuid         not null references management.employee (id),
    work_location varchar(255) not null,
    start_date    date         not null check ( start_date >= current_date ),
    salary        money        not null,
    currency      varchar(3)   not null check ( length(currency) = 3 ),
    created_at    timestamp    not null default current_timestamp,
    updated_at    timestamp    not null default current_timestamp
);

create table management.emergency_information
(
    id                uuid primary key default gen_random_uuid(),
    employee_id       uuid           not null references management.employee (id) on delete cascade,
    address           varchar(255),
    -- references: https://en.wikipedia.org/wiki/Telephone_numbers_in_the_Philippines#:~:text=Mobile%20phone%20numbers%20are%20always,a%20seven%2Ddigit%20number).
    tel_number        varchar(10)    null check (tel_number ~ '^0[0-9]{2}-[0-9]{3}-[0-9]{4}$' or
                                                 tel_number ~ '^02-[0-9]{4}-[0-9]{4}$'),
    /*
        Mobile numbers in the Philippines are 10 or 12 digits long.
        Local mobile numbers
            Can be 11 digits long, such as 09171234567
            Can be 12 digits long, such as 639181234567
            Do not include special characters like + - . #
    */
    mobile_number     varchar(12)    null check (mobile_number ~ '^(09[0-9]{9}|639[0-9]{9})$'),
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
);

-- triggers
create function management.create_basic_user_employee_trigger_fn() returns trigger as
$$
declare
    _email    varchar(255);
    _password varchar(255);
    _user_id  uuid;
begin

    -- password format. last_name.m/d/y
    _password = new.last_name || date_part('month', new.birth_date) || date_part('day', new.birth_date) ||
                date_part('year', new.birth_date);

    if new.email is null then
        -- create a basic user with email format of last_name.random_number@domain
        _email := new.last_name || floor(random() * 100000)::int || current_setting('app.management.domain');
    else
        _email := new.email;
    end if;

    insert into auth.basic_user(email, password) values (_email, _password) returning user_id into _user_id;

    new.auth_user_id = _user_id;

    return new;
end
$$ language plpgsql;

create trigger create_basic_user_employee_trigger
    before insert
    on management.employee
    for each row
execute function management.create_basic_user_employee_trigger_fn();

-- policies

-- employee table

-- insert policy: only admin can create employee

-- read policy: only admin or manager can view other employee information

-- read policy: only the current employee can view its own employee information.

-- delete policy only admin can remove employee

-- department table

-- insert policy: only admin can create department

-- insert policy: only admin can add employee to different departments

-- read policy: employees under a department can see its information

-- update policy: only admin can update department

-- delete policy: only admin can remove department.

-- job information:

-- insert policy: only admin can create job information

-- read policy: admin, managers, and employees can see each other's job information

-- update policy: only admin can update job information

-- delete policy: only admin can delete job information

-- emergency information:

-- insert policy: admin can create emergency information to all employees.

-- read policy: admins, managers and employee can see emergency information.

-- update policy: admin or the current_user (employee) can update its emergency information.

-- delete policy: only admins can delete emergency information. note: do not do this if possible.

-- personnel action:

-- insert policy: anyone can request pan as long as 'Pending' is supplied.

-- read policy: only admin can read pan information for different employees

-- read policy: only the current user can read its own pan information.

-- update policy: only admin can update pan information for different employees

-- delete policy: no one can delete personnel action to comply with work ethics.