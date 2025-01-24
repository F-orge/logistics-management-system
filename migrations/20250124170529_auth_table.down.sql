-- Add down migration script here
drop table "auth"."user" cascade;

drop table "auth"."basic_user" cascade;