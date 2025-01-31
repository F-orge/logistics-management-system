-- Add down migration script here
drop trigger "storage_file_before_insert_trigger" on "storage"."file";

drop function "storage"."insert_storage_file_trigger_fn";
