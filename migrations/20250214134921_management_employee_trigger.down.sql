-- Add down migration script here
drop trigger "management_employee_after_insert_trigger" on "management"."employee";
drop function "management"."insert_management_employee_trigger_fn";

drop trigger "management_employee_after_update_trigger" on "management"."employee";
drop function "management"."update_management_employee_trigger_fn";