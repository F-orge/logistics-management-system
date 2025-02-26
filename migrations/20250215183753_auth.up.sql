-- Add up migration script here
create schema logistics;

create type auth_type as enum('basic_auth');

create table
    logistics.users (
        id uuid not null primary key default gen_random_uuid (),
        auth_type auth_type not null,
        email varchar(255) not null unique check (email ~ '^[^@]+@[^@]+\.[^@]+$'),
        password varchar(128) not null check (
            length(password) > 8
            and password ~ '[A-Z]'
            and password ~ '[0-9]'
            and password ~ '[^a-zA-Z0-9]'
        ),
        create_at timestamp not null default current_timestamp,
        updated_at timestamp not null default current_timestamp
    );