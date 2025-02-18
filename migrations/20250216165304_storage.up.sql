-- Add up migration script here
create schema storage;

grant usage on schema storage to web;

create table
    storage.file (
        id uuid primary key default gen_random_uuid (),
        name varchar(256) not null,
        type varchar not null,
        size int not null,
        is_public bool not null default false,
        owner_id uuid null references auth.users (id) on delete cascade,
        created_at timestamp not null default current_timestamp,
        updated_at timestamp not null default current_timestamp
    );

create table
    storage.file_access (
        user_id uuid references auth.users (id) on delete cascade,
        file_id uuid references storage.file (id) on delete cascade,
        primary key (user_id, file_id)
    );

-- triggers
create
or replace function public.update_timestamp () returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language plpgsql;

create trigger storage_file_update_timestamp before
update on storage.file for each row
execute function public.update_timestamp ();

create function storage.insert_owner_id_trigger_fn () returns trigger as $$
begin 
    new.owner_id = auth.uid();
    return new;
end 
$$ language plpgsql;

create trigger storage_file_insert_owner_id_trigger before insert on storage.file for each row
execute function storage.insert_owner_id_trigger_fn ();

create function storage.insert_allow_access_trigger_fn () returns trigger as $$
begin
    insert into storage.file_access(user_id, file_id) values (auth.uid(), new.id);
    return new;
end
$$ language plpgsql;

create trigger storage_file_insert_allow_access_trigger
after insert on storage.file for each row
execute function storage.insert_allow_access_trigger_fn ();

grant all on table storage.file,
storage.file_access to web;

-- policies
alter table storage.file enable row level security;

alter table storage.file_access enable row level security;

-- insert policy. only web can upload files to the system
create policy "only web can insert files" on storage.file for insert
with
    check (current_user = 'web');

-- read policy. users can read public files
create policy "users can read public files" on storage.file for
select
    using (is_public = true);

-- read policy. users can read their own files
create policy "users can read their own files" on storage.file for
select
    using (owner_id = auth.uid ());

-- read policy. users can read shared files to them
create policy "users can read shared files" on storage.file for
select
    using (
        exists (
            select
                1
            from
                storage.file_access
            where
                file_access.file_id = file.id
                and file_access.user_id = auth.uid ()
        )
    );

-- update policy. only web can update files
create policy "only web can update files" on storage.file for
update using (current_user = 'web');

-- delete policy. only web can delete files
create policy "only web can delete files" on storage.file for delete using (current_user = 'web');

-- insert policy: web can insert file access
create policy "web can insert file access" on storage.file_access for insert
with
    check (current_user = 'web');

-- insert policy: current user can share its own file to others
create policy "current user can share own files" on storage.file_access for insert
with
    check (user_id = auth.uid ());

-- read policy: users can read shared file access to them
create policy "users can read shared file access" on storage.file_access for
select
    using (user_id = auth.uid ());

-- delete policy: web can delete file access to a file
create policy "web can delete file access" on storage.file_access for delete using (current_user = 'web');