-- Add up migration script here
create function "auth"."insert_basic_user_trigger_fn"() returns trigger as
$$
declare
    new_id uuid;
begin

    -- create a user
    insert into "auth"."user" (auth_type) values ('basic_auth') returning id into new_id;

    new.user_id := new_id;
    new.password := crypt(new.password, gen_salt('bf'));

    return new;
end;
$$ language plpgsql;

-- insert trigger
create trigger "auth_basic_user_before_insert_trigger"
    before insert
    on "auth"."basic_user"
    for each row
execute function "auth"."insert_basic_user_trigger_fn"();

-- update trigger function
create function "auth"."update_basic_user_trigger_fn"() returns trigger as
$$
begin

    -- update the `updated_at column`
    update "auth"."user" set updated_at = now() where id = new.user_id;

    if new.password is distinct from old.password then
        new.password := crypt(new.password, gen_salt('bf'));
    end if;

    return new;
end;
$$ language plpgsql;

-- update trigger function
create trigger "auth_basic_user_before_update_trigger"
    before
        update
    on "auth"."basic_user"
    for each row
execute function "auth"."update_basic_user_trigger_fn"();

-- test
do
$$
    declare
    begin
        -- setup
        set role web;

        -- test
        insert into "auth"."basic_user"(email, password)
        values ('sample@email.com', 'Randompassword1!');

        -- check insert user
        if (select user_id from "auth"."basic_user" where email = 'sample@email.com') is null then
            raise exception 'user_id does not exists';
        end if;

        -- check if encrypted
        if not exists (select 1 from "auth"."basic_user" where password != 'Randompassword1!') then
            raise exception 'password is not encrypted';
        end if;

        -- check if password match with the database
        if not exists (select 1 from "auth"."basic_user" where password = crypt('Randompassword1!', password)) then
            raise exception 'current password does not match with the database';
        end if;

        -- update password
        update "auth"."basic_user" set password = 'NewPassword1!' where email = 'sample@email.com';

        -- check if updated password is encrypted
        if not exists (select 1 from "auth"."basic_user" where password != 'NewPassword1!') then
            raise exception 'updated password is not encrypted';
        end if;

        -- check if updated password match with the database
        if not exists (select 1 from "auth"."basic_user" where password = crypt('NewPassword1!', password)) then
            raise exception 'updated current password does not match with the database';
        end if;

        -- check if updated_at is being updated properly
        if not exists (select 1
                       from "auth"."basic_user"
                                inner join auth."user" u on u.id = basic_user.user_id
                       where created_at = updated_at) then
            raise exception 'updated_at column is not properly set';
        end if;

        -- clean
        truncate table "auth"."basic_user" cascade;
        truncate table "auth"."user" cascade;
        set role postgres;
    exception
        when others then
            perform 'rollback';
            raise;
    end
$$ language plpgsql;