-- Add up migration script here
create view
    "auth"."basic_user_view" as
(
select user_id                   as id,
       email,
       "auth"."user"."auth_type" as auth_type,
       created_at,
       updated_at
from "auth"."basic_user"
         inner join "auth"."user" on "auth"."user"."id" = "auth"."basic_user"."user_id"
    );

grant
    select
    on "auth"."basic_user_view" to web;

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

        -- test view
        select * from "auth"."basic_user_view";

        -- clean
        truncate table "auth"."basic_user" cascade;
        truncate table "auth"."user" cascade;
        set role postgres;
    end
$$ language plpgsql;

