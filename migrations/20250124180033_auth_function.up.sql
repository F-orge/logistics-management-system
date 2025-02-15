/* plpgsql-language-server:disable validation */
-- Add up migration script here
create function "auth"."basic_login"(email varchar, password varchar, out token text) as
$$
declare
    _user_id uuid;
begin

    select user_id
    from "auth"."basic_user"
    where "basic_user"."email" = "basic_login"."email"
      and "basic_user"."password" = crypt("basic_login"."password", "basic_user"."password")
    into _user_id;

    if _user_id is null then
        raise invalid_password using message = 'Invalid email or password';
    end if;

    select sign(
                   row_to_json(r), current_setting('app.jwt_secret')
           ) as token
    from (select _user_id             as id,
                 "basic_login".email  as email,
                 'basic_auth'         as auth_type,
                 extract(
                         epoch
                         from now()
                 )::integer + 60 * 60 as exp) r
    into token;
end;
$$ language plpgsql;

create function "auth"."basic_update_password"(
    email varchar,
    password varchar,
    new_password varchar
) returns void as
$$
declare
    _user_id uuid;
begin

    select user_id
    from "auth"."basic_user"
    where "basic_user"."email" = "basic_update_password"."email"
      and "basic_user"."password" = crypt("basic_update_password"."password", "basic_user"."password")
    into _user_id;

    if _user_id is null then
        raise invalid_password using message = 'Invalid email or password';
    end if;

    update "auth"."basic_user"
    set password = crypt("basic_update_password"."new_password", gen_salt('bf'))
    where user_id = _user_id;
end;
$$ language plpgsql;

create function "auth"."current_user"() returns json as
$$
declare
begin
    return (select payload from verify(current_setting('request.jwt'), current_setting('app.jwt_secret')));
end;
$$ language plpgsql;

create function "auth"."uid"() returns uuid as
$$
begin
    return (select ("auth"."current_user"())::json ->> 'id'::uuid);
end
$$ language plpgsql;

create function "auth"."email"() returns varchar as
$$
begin
    return (select ("auth"."current_user"())::json ->> 'email'::varchar);
end
$$ language plpgsql;

create function "auth"."auth_type"() returns varchar as
$$
begin
    return (select ("auth"."current_user"())::json ->> 'auth_type'::varchar);
end
$$ language plpgsql;

-- test
do
$$
    declare
        _user_id        uuid;
        _user_jwt_token text;
    begin
        -- setup
        set role web;
        perform set_config('app.jwt_secret', 'secret', true);

        -- test
        insert into "auth"."user"(auth_type) values ('basic_auth') returning id into _user_id;
        insert into "auth"."basic_user"(email, password, user_id)
        values ('sample@email.com', 'RandomPassword1', _user_id);

        -- test login
        select "auth"."basic_login"('sample@email.com', 'RandomPassword1') into _user_jwt_token;

        if not (
            (select (payload::json ->> 'id') = _user_id
             from verify(_user_jwt_token, current_setting('app.jwt_secret')))) then
            raise exception 'JWT does not match';
        end if;

        -- test update password
        perform "auth"."basic_update_password"('sample@email.com', 'RandomPassword1', 'NewPassword1');
    exception
        when others then
            perform 'rollback';
            raise;
    end
$$ language plpgsql;