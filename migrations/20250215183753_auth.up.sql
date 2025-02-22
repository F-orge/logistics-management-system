-- Add up migration script here
create schema auth;

grant usage on schema auth to web;

create type auth.auth_type as enum('basic_auth');

create table
    auth.users (
        id uuid not null primary key default gen_random_uuid (),
        auth_type auth.auth_type not null
    );

create table
    auth.basic_user (
        email varchar(255) not null unique check (email ~ '^[^@]+@[^@]+\.[^@]+$'),
        password varchar(128) not null check (
            length(password) > 8
            and password ~ '[A-Z]'
            and password ~ '[0-9]'
            and password ~ '[^a-zA-Z0-9]'
        ),
        user_id uuid references auth.users (id) on delete cascade,
        create_at timestamp not null default current_timestamp,
        updated_at timestamp not null default current_timestamp
    );

create function auth.insert_basic_auth_trigger_fn () returns trigger as $$
declare
    _user_id uuid;
begin
    insert into auth.users(auth_type) values ('basic_auth') returning id into _user_id;
    new.user_id := _user_id;
    if tg_op = 'INSERT' or new.password <> old.password then
        new.password := crypt(new.password, gen_salt('bf'));
    end if;
    return new;
end
$$ language plpgsql;

create trigger "auth_users_before_insert_trigger" before insert on auth.basic_user for each row
execute function auth.insert_basic_auth_trigger_fn ();

-- view
create view
    auth.basic_user_view as (
        select
            bu.*,
            u.auth_type,
            u.id as id
        from
            auth.basic_user bu
            inner join auth.users u ON u.id = bu.user_id
    );

grant
select
    on auth.basic_user_view to web;

-- functions
create function auth.basic_user_login (email varchar, password varchar, out token text) as $$
declare
    _user_id uuid;
begin
    select id
    from auth.basic_user_view
    where auth.basic_user_view.email = basic_user_login.email
      and auth.basic_user_view.password = crypt(basic_user_login.password, auth.basic_user_view.password)
    into _user_id;

    if _user_id is null then 
        raise invalid_authorization_specification using message = 'invalid email or password';
    end if;

    select sign(
                   payload := row_to_json(r),
                   secret := current_setting('app.jwt.secret')
           )
    from (select _user_id                                                                        as sub, -- subject
                 current_setting('app.jwt.issuer')                                               as iss, -- issuer
                 current_setting('app.jwt.audience')                                             as aud, -- audience
                 extract(epoch from now())::integer + (current_setting('app.jwt.expiry')::integer) as exp, -- expiry
                 now()                                                                           as iat, -- issued at time
                 gen_random_uuid()                                                               as jti -- jwt id
         ) r
    into token;
end
$$ language plpgsql;

create function auth.user () returns table (
    sub uuid,
    iss name,
    aud varchar,
    exp integer,
    iat timestamp,
    jti uuid
) as $$
declare
    _payload json;
begin
    select payload from verify(current_setting('request.jwt.token'), current_setting('app.jwt.secret')) into _payload;

    if (_payload ->> 'iss')::name != current_user then
        raise invalid_authorization_specification using message = 'invalid issuer tag';
    end if;

    -- TODO: implement jti in the future
    if (_payload ->> 'aud')::varchar != current_setting('app.jwt.audience') then
        raise invalid_authorization_specification using message = 'invalid audience tag';
    end if;

    if to_timestamp((_payload ->> 'exp')::integer) < now() then
        raise invalid_authorization_specification using message = 'token has expired';
    end if;

    if _payload ->> 'sub' is null then 
        raise invalid_authorization_specification using message = 'no token provided'; 
    end if;

    return query (select (_payload ->> 'sub')::uuid      as sub,
                         (_payload ->> 'iss')::name      as iss,
                         (_payload ->> 'aud')::varchar   as aud,
                         (_payload ->> 'exp')::integer   as exp,
                         (_payload ->> 'iat')::timestamp as iat,
                         (_payload ->> 'jti')::uuid      as jti);
end
$$ language plpgsql;

create function auth.uid () returns uuid as $$
declare
    _payload json;
begin 
    return (select sub from auth."user"());
end $$ language plpgsql;

grant all privileges on schema auth to web;

grant all on table auth.basic_user,
auth.users to web;

alter table auth.users enable row level security;

alter table auth.basic_user enable row level security;

-- insert policy: current user can insert users
create policy "web can insert user" on auth.users as permissive for insert to web
with
    check (true);

-- insert policy: web can insert basic_user
create policy "web can insert basic_user" on auth.basic_user as permissive for insert to web
with
    check (true);

-- read policy: web can read basic_user
create policy "web can read basic_user" on auth.basic_user as permissive for
select
    to web using (true);

-- read policy: web can read users
create policy "web can read users" on auth.users as permissive for
select
    to web using (true);

-- update policy: web can update basic_user information
create policy "web can update user information" on auth.basic_user as permissive for
update to web using (true);

-- update policy: web can update users information
create policy "web can update users information" on auth.users as permissive for
update to web using (true);

-- update policy: current user can update its own information
create policy "current user can update its own information" on auth.basic_user as permissive for
update to web using (user_id = auth.uid ());

-- delete policy: web can delete user
create policy "web can delete user" on auth.users as permissive for delete to web using (true);

-- delete policy: current user can delete its own account
create policy "current user can delete its own account" on auth.users as permissive for delete to web using (id = auth.uid ());