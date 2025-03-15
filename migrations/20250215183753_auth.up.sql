-- Add up migration script here
create schema logistics;

create table
    logistics.users (
        id uuid not null primary key default gen_random_uuid (),
        auth_type varchar not null check (auth_type in ('basic_auth')),
        email varchar(255) not null unique check (email ~ '^[^@]+@[^@]+\.[^@]+$'),
        _password varchar not null check (
            length(_password) > 8
            and _password ~ '[A-Z]'
            and _password ~ '[0-9]'
            and _password ~ '[^a-zA-Z0-9]'
        ),
        created timestamp not null default current_timestamp,
        updated timestamp not null default current_timestamp
    )