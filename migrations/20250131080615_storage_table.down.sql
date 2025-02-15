/* plpgsql-language-server:disable validation */
-- Add down migration script here
drop table "storage"."file_access" cascade;

drop table "storage"."file" cascade;

