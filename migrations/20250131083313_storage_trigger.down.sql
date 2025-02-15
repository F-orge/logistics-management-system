/* plpgsql-language-server:disable validation */
-- Add down migration script here
drop trigger "storage_file_after_insert_trigger" on "storage"."file";

drop function "storage"."insert_storage_file_trigger_fn";