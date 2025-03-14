-- Add up migration script here
create schema logistics;

create type logistics.auth_type as enum('basic_auth');

create table
    logistics.users (
        id uuid not null primary key default gen_random_uuid (),
        auth_type logistics.auth_type not null,
        email varchar(255) not null unique check (email ~ '^[^@]+@[^@]+\.[^@]+$'),
        _password varchar(128) not null check (
            length(_password) > 8
            and _password ~ '[A-Z]'
            and _password ~ '[0-9]'
            and _password ~ '[^a-zA-Z0-9]'
        ),
        created timestamp not null default current_timestamp,
        updated timestamp not null default current_timestamp
    )