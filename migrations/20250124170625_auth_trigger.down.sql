-- Add down migration script here
drop trigger "auth_basic_user_before_insert_trigger" on "auth"."basic_user";

drop function "auth"."insert_basic_user_trigger_fn";

drop trigger "auth_basic_user_before_update_trigger" on "auth"."basic_user";

drop function "auth"."update_basic_user_trigger_fn";