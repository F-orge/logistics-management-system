-- Add up migration script here
create function "storage"."insert_storage_file_trigger_fn"() returns trigger as $$
begin

  -- update table and add the owner_id
  update "storage"."file" set owner_id = current_setting('request.jwt.claim.user_id')::uuid where id = new.id;

  return new;
end
$$ language plpgsql;

-- insert trigger
create trigger "storage_file_before_insert_trigger" before insert on "storage"."file" for each row execute function "storage"."insert_storage_file_trigger_fn"();