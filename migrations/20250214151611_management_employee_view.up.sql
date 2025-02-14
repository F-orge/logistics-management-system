-- Add up migration script here
create view "management"."employee_view" as (
    select employee.*,email,auth_type from "management"."employee" inner join "auth"."basic_user_view" on user_id = "auth"."basic_user_view"."id"
);

grant select on "management"."employee_view" to nextjs;
