-- Add up migration script here
do $$
declare
  _user_token varchar;
begin
  -- setup
  set role nextjs;
  perform set_config('app.jwt_secret','secret',true);
  -- test
  
  -- insert
  insert into "auth"."basic_user"(email,password) values ('sample@email.com','random password');

  -- insert check: check if basic_user is inserted
  if not exists (select 1 from "auth"."basic_user" where email = 'sample@email.com') then
    raise exception 'insert check failed';
  end if;

  if exists (select 1 from "auth"."basic_user" where email = 'sample@email.com' and password = 'random password') then
    raise exception 'db password and password match and is not encrypted';
  end if;

  -- select view check: check if basic_user has auth_type
  if not exists (select 1 from "auth"."basic_user_view" where email = 'sample@email.com' and auth_type = 'basic_auth') then
    raise exception 'basic user doesnt match with the view';
  end if; 

  -- update password
  update "auth"."basic_user" set password = 'new password' where email = 'sample@email.com';

  if not exists (select 1 from "auth"."basic_user" where email = 'sample@email.com' and password = crypt('new password',password)) then 
    raise exception 'updated password doesnt match';
  end if;

  -- delete user
  delete from "auth"."basic_user" where email = 'sample@email.com' and password = crypt('new password',password);

  if exists (select 1 from "auth"."basic_user" where email = 'sample@email.com') then 
    raise exception 'user is not deleted';
  end if;

  -- re insert again
  insert into "auth"."basic_user"(email,password) values ('sample@email.com','random password');

  -- login
  select "auth"."basic_login"('sample@email.com','random password') into _user_token;
  
  perform set_config('request.jwt',_user_token,true);

  -- check current user
  if not exists (select 1 from "auth"."basic_user" where "auth"."current_user"()::json->>'email' = 'sample@email.com') then 
    raise exception 'current_user doesnt exists';
  end if;

  -- clean
  set role postgres;
  perform 'rollback';
exception
  when others then
    perform 'rollback';
    raise;
end $$ language plpgsql;