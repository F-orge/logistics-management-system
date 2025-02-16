-- Add up migration script here
create function "auth"."basic_login"(email varchar,password varchar, out token text) as $$
declare
  _user_id uuid;
begin

  select user_id from "auth"."basic_user" where "basic_user"."email" = "basic_login"."email" and "basic_user"."password" = crypt("basic_login"."password","basic_user"."password") into _user_id;

  if _user_id is null then
    raise invalid_password using message = 'Invalid email or password';
  end if;
  
  select "pgjwt"."sign"(
        row_to_json(r), current_setting('app.jwt_secret')
    ) as token
  from (
      select _user_id as id, "basic_login".email as email, extract(
          epoch
          from now()
      )::integer + 60 * 60 as exp
  ) r into token;
end;
$$ language plpgsql;

-- schema test