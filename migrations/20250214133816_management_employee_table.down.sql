-- Add down migration script here
drop table "management"."employee" cascade;
drop type "management"."role" cascade;