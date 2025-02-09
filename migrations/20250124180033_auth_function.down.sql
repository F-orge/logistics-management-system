-- Add down migration script here
drop function "auth"."basic_login";
drop function "auth"."basic_update_password";