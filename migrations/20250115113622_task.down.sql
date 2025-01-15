-- Add down migration script here

drop table "etmar_logistics"."task" cascade;

drop type "etmar_logistics"."task_status_enum" cascade;