-- Add up migration script here
-- policies
-- create employee, only super_admin and manager can create employee
-- read employee, anyone can read employee information
-- update employee, only super_admin and the employee itself can update employee information
-- remove employee, only super_admin can remove employee

alter table "management"."employee" enable row level security;

create policy "SUPER_ADMIN or MANAGER can create employee" on "management"."employee" for insert to nextjs with check (
    exists (select 1 from "management"."employee_view" where "auth"."current_user"()::json->>'email' = email and role in ('SUPER_ADMIN','MANAGER'))
);

create policy "Any employee can read each other's information" on "management"."employee" for select to nextjs using (
    ("auth"."current_user"()::json->>'id')::uuid in (select user_id from "management"."employee_view")
);

create policy "SUPER_ADMIN or the current user can update employee information" on "management"."employee" for update to nextjs using (
    exists (select 1 from "management"."employee_view" where "auth"."current_user"()::json->>'email' = email and role = 'SUPER_ADMIN')
    or ("auth"."current_user"()::json->>'id')::uuid = user_id
);

create policy "SUPER_ADMIN can remove employee" on "management"."employee" for update to nextjs using (
    exists (select 1 from "management"."employee_view" where "auth"."current_user"()::json->>'email' = email and role = 'SUPER_ADMIN')
);