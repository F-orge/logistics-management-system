-- Add up migration script here
-- reference: https://www.indeed.com/recruitment/c/info/employee-information-form
-- reference: https://in.indeed.com/career-advice/starting-new-job/employee-information-form
create type logistics.employee_marital_status as enum(
    'Single',
    'Married',
    'Divorced',
    'Widowed',
    'Separated'
);

create type logistics.employee_role as enum('Admin', 'Manager', 'Employee');

create type logistics.employee_status as enum('Active', 'Inactive');

create type logistics.employee_contract_type as enum('FullTime', 'PartTime');

create table
    logistics.employee (
        id uuid primary key default gen_random_uuid (),
        -- profile picture
        avatar_id uuid null,
        -- profile cover photo
        cover_photo_id uuid null,
        -- note:
        -- full name must be unique so no duplicate information will be processed
        first_name varchar(255) not null,
        middle_name varchar(255) not null,
        last_name varchar(255) not null,
        constraint unique_employee_name unique (first_name, middle_name, last_name),
        -- references: https://en.wikipedia.org/wiki/Telephone_numbers_in_the_Philippines#:~:text=Mobile%20phone%20numbers%20are%20always,a%20seven%2Ddigit%20number).
        tel_number varchar(10) null,
        /*
        Mobile numbers in the Philippines are 10 or 12 digits long.
        Local mobile numbers
        Can be 11 digits long, such as 09171234567
        Can be 12 digits long, such as 639181234567
        Do not include special characters like + - . #
         */
        mobile_number varchar(12) null check (mobile_number ~ '^(09[0-9]{9}|639[0-9]{9})$'),
        -- note:
        -- this will be the same in auth.basic_user. if auth_type is different use any email
        email varchar(255) unique check (email ~ '^.+@.+\..+$'),
        role logistics.employee_role not null default 'Employee',
        status logistics.employee_status not null default 'Active',
        contract_type logistics.employee_contract_type not null,
        -- note:
        -- philippine national id format is xxxx-xxxx-xxxx-xxxx
        phil_nat_id varchar(19) not null check (
            phil_nat_id ~ '^[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{4}$'
        ),
        birth_date date not null check (birth_date <= current_date - interval '18 years'),
        special_interests varchar(255) [] null,
        learning_institutions varchar(255) [] not null,
        auth_user_id uuid not null references logistics.users (id) on delete cascade,
        spouse_first_name varchar(255) null,
        spouse_middle_name varchar(255) null,
        spouse_last_name varchar(255) null,
        constraint unique_spouse_name unique (
            spouse_first_name,
            spouse_middle_name,
            spouse_last_name
        ),
        spouse_employer varchar(255) null,
        created_at timestamp not null default current_timestamp,
        updated_at timestamp not null default current_timestamp
    );

create table
    logistics.department (
        id uuid primary key default gen_random_uuid (),
        name varchar(255) not null,
        -- note:
        -- markdown compatible text format
        description text null,
        employees uuid[] not null,
        created_at timestamp not null default current_timestamp,
        updated_at timestamp not null default current_timestamp
    );

create table
    logistics.job_information (
        id uuid primary key default gen_random_uuid (),
        title varchar(255) not null,
        employee_id uuid not null references logistics.employee (id),
        department_id uuid not null references logistics.department (id),
        supervisor_id uuid not null references logistics.employee (id),
        work_location varchar(255) not null,
        start_date date not null check (start_date >= current_date),
        salary money not null,
        currency varchar(3) not null check (length(currency) = 3),
        created_at timestamp not null default current_timestamp,
        updated_at timestamp not null default current_timestamp
    );

create table
    logistics.emergency_information (
        id uuid primary key default gen_random_uuid (),
        employee_id uuid not null references logistics.employee (id) on delete cascade,
        address varchar(255),
        -- references: https://en.wikipedia.org/wiki/Telephone_numbers_in_the_Philippines#:~:text=Mobile%20phone%20numbers%20are%20always,a%20seven%2Ddigit%20number).
        tel_number varchar(10) null check (
            tel_number ~ '^0[0-9]{2}-[0-9]{3}-[0-9]{4}$'
            or tel_number ~ '^02-[0-9]{4}-[0-9]{4}$'
        ),
        /*
        Mobile numbers in the Philippines are 10 or 12 digits long.
        Local mobile numbers
        Can be 11 digits long, such as 09171234567
        Can be 12 digits long, such as 639181234567
        Do not include special characters like + - . #
         */
        mobile_number varchar(12) null check (mobile_number ~ '^(09[0-9]{9}|639[0-9]{9})$'),
        contact_name varchar(255) not null,
        health_conditions varchar(255) [] null
    );