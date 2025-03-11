-- Add up migration script here
create table
  logistics.permissions (
    id uuid not null primary key default gen_random_uuid (),
    user_id uuid not null references logistics.users (id) on delete cascade,
    entity_name text not null,
    action text not null check (action in ('create', 'read', 'update', 'delete'))
  );