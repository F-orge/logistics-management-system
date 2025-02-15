-- Add up migration script here
create table
    "auth"."user"
(
    id         uuid primary key   default gen_random_uuid(),
    auth_type  varchar   not null check ( auth_type in ('basic_auth') ),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table
    "auth"."basic_user"
(
    email    varchar(255) primary key check ( email like '%' || '@' || '%' ),
    password varchar(128) not null check ( length(password) > 8 and password ~ '[A-Z]' and password ~ '[0-9]' and
                                           password ~ '[^a-zA-Z0-9]' ),
    user_id  uuid,
    foreign key (user_id) references "auth"."user" (id) on delete cascade
);

grant all on "auth"."user" to web;

grant all on "auth"."basic_user" to web;

-- test
do
$$
    declare
        _user_id uuid;
    begin
        -- setup
        set role web;

        -- test
        insert into "auth"."user"(auth_type) values ('basic_auth') returning id into _user_id;
        insert into "auth"."basic_user"(email, password, user_id)
        values ('sample@email.com', 'Randompassword1!', _user_id);

        -- clean
        truncate table "auth"."basic_user" cascade;
        truncate table "auth"."user" cascade;
        set role postgres;
    end
$$ language plpgsql;