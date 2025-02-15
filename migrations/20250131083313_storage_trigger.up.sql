/* plpgsql-language-server:disable validation */
-- Add up migration script here
create function "storage"."insert_storage_file_trigger_fn" () returns trigger as $$
begin
  -- update table and add the owner_id
  insert into "storage"."file_access"(file_id,user_id) values (new.id,("auth"."current_user"()::json->>'id')::uuid);
  return new;
end
$$ language plpgsql;

-- insert trigger
create trigger "storage_file_after_insert_trigger"
after insert on "storage"."file" for each row
execute function "storage"."insert_storage_file_trigger_fn" ();