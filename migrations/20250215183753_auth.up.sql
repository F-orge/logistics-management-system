-- Add up migration script here
create schema auth;

grant usage on schema auth to web;

create type auth.auth_type as enum ('basic_auth');

create table auth.users
(
    id        uuid           not null primary key default gen_random_uuid(),
    auth_type auth.auth_type not null,
    role      name
);

create table auth.basic_user
(
    email      varchar(255) not null unique check ( email ~ '^.+@.+\..+$'),
    password   varchar(128) not null check ( length(password) > 8 and password ~ '[A-Z]' and password ~ '[0-9]' and
                                             password ~ '[^a-zA-Z0-9]' ),
    user_id    uuid references auth.users (id) on delete cascade,
    create_at  timestamp    not null default current_timestamp,
    updated_at timestamp    not null default current_timestamp
);

create function auth.insert_basic_auth_trigger_fn() returns trigger as
$$
declare
    _user_id uuid;
begin
    insert into auth.users(auth_type, role) values ('basic_auth', 'anon') returning id into _user_id;
    new.user_id := _user_id;
    if tg_op = 'INSERT' or new.password <> old.password then
        new.password := crypt(new.password, gen_salt('bf'));
    end if;
    return new;
end
$$ language plpgsql;

create trigger "auth_users_before_insert_trigger"
    before insert
    on auth.basic_user
    for each row
execute function auth.insert_basic_auth_trigger_fn();

create function auth.check_role_exists() returns trigger as
$$
begin
    if not exists (select 1 from pg_roles as r where r.rolname = new.role) then
        raise foreign_key_violation using message = 'unknown database role: ' || new.role;
    end if;
    return new;
end
$$ language plpgsql;

create constraint trigger "auth_user_check_roles_trigger"
    after insert or update
    on auth.users
    for each row
execute procedure auth.check_role_exists();

-- view
create view auth.basic_user_view as
(
select bu.*, u.auth_type, u.id as id, u.role as role
from auth.basic_user bu
         inner join auth.users u ON u.id = bu.user_id
    );

grant select on auth.basic_user_view to web;

-- functions
create function auth.user_role(email varchar, password varchar) returns name as
$$
begin
    return (select role
            from auth.basic_user_view u
            where u.email = user_role.email
              and u.password = crypt(user_role.password, u.password));
end
$$ language plpgsql;

create function auth.basic_user_login(email varchar, password varchar, out token text) as
$$
declare
    _user_id uuid;
begin
    select id
    from auth.basic_user_view
    where auth.basic_user_view.email = basic_user_login.email
      and auth.basic_user_view.password = crypt(basic_user_login.password, auth.basic_user_view.password)
    into _user_id;
    select sign(
                   payload := row_to_json(r),
                   secret := current_setting('app.jwt.secret')
           )
    from (select _user_id                                                                        as sub, -- subject
                 current_setting('app.jwt.issuer')                                               as iss, -- issuer
                 current_setting('app.jwt.audience')                                             as aud, -- audience
                 extract(epoch from now())::integer + current_setting('app.jwt.expiry')::integer as exp, -- expiry
                 now()                                                                           as iat, -- issued at time
                 gen_random_uuid()                                                               as jti -- jwt id
         ) r
    into token;
end
$$ language plpgsql;

create function auth.current_user()
    returns table
            (
                sub uuid,
                iss name,
                aud varchar,
                exp integer,
                iat timestamp,
                jti uuid
            )
as
$$
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

    return query (select (_payload ->> 'sub')::uuid      as sub,
                         (_payload ->> 'iss')::name      as iss,
                         (_payload ->> 'aud')::varchar   as aud,
                         (_payload ->> 'exp')::integer   as exp,
                         (_payload ->> 'iat')::timestamp as iat,
                         (_payload ->> 'jti')::uuid      as jti);
end
$$ language plpgsql;

grant all privileges on schema auth to web;
grant all on table auth.basic_user,auth.users to web;

-- test
/*
    set role web;
    insert into auth.basic_user(email, password)
    values ('sample@email.com', 'RandomPassword1!');
    select set_config('app.jwt.secret', 'secret', false);
    select set_config('app.jwt.issuer', current_user, false);
    select set_config('app.jwt.audience', 'management.com', false);
    select set_config('app.jwt.expiry', '300', false);
    select set_config('request.jwt.token', auth.basic_user_login('sample@email.com', 'RandomPassword1!'), false);
    select *
    from auth.current_user();
    set role postgres;
*/
