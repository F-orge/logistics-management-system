-- Add up migration script here

-- insert trigger function
create function "management"."insert_management_employee_trigger_fn"() returns trigger as $$
declare
  generated_email varchar;
  generated_password varchar;
  _user_id uuid;
begin 
  -- password format: month/day/year/lastname
  -- email format: last_name.first_name.auto-number@domain
  select new.last_name || '.' || new.first_name || '.' || floor(random() * 100000 + 1)::int || '@' || current_setting('app.domain')::varchar into generated_email;
  select date_part('month',new.birth_date)::int::varchar || date_part('day',new.birth_date)::int::varchar || date_part('year',new.birth_date)::int::varchar || new.last_name into generated_password;
  
  insert into "auth"."basic_user"(email,password) values (generated_email, generated_password);

  select id from "auth"."basic_user_view" where email = generated_email limit 1 into _user_id;

  update "management"."employee" set user_id = _user_id where id = new.id;

  return new;
end
$$ language plpgsql;

-- insert trigger
create trigger "management_employee_after_insert_trigger" after insert on "management"."employee" for each row execute function "management"."insert_management_employee_trigger_fn"();

-- update trigger function
create function "management"."update_management_employee_trigger_fn"() returns trigger as $$
begin

  -- update the `updated_at column`
  new.updated_at = now();

  return new;
end;
$$ language plpgsql;

-- update trigger function
create trigger "management_employee_after_update_trigger" after
update
  on "management"."employee" for each row execute function "management"."update_management_employee_trigger_fn"();