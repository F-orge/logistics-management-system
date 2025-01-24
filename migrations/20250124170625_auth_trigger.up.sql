-- Add up migration script here
create function "auth"."insert_basic_user_trigger_fn"() returns trigger as $$
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
create trigger "auth_basic_user_before_insert_trigger" before insert on "auth"."basic_user" for each row execute function "auth"."insert_basic_user_trigger_fn"();

-- update trigger function
create function "auth"."update_basic_user_trigger_fn"() returns trigger as $$
begin
  
  -- update the `updated_at column`
  update "auth"."user" set updated_at = now() where id = new.user_id;
  
  new.password := crypt(new.password, gen_salt('bf'));

  return new;
end;
$$ language plpgsql;

-- update trigger function
create trigger "auth_basic_user_before_update_trigger" before
update
  on "auth"."basic_user" for each row execute function "auth"."update_basic_user_trigger_fn"();