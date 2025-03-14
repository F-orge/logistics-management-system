-- Add up migration script here
create table
    logistics.file (
        id uuid primary key default gen_random_uuid (),
        name varchar(256) not null,
        file_path text not null,
        is_public bool not null default false,
        owner_id uuid not null references logistics.users (id) on delete cascade,
        shared_to uuid[] not null,
        created timestamp not null default current_timestamp,
        updated timestamp not null default current_timestamp
    );