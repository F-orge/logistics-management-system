-- Add up migration script here
create view "auth"."basic_user_view" as (
  select
    user_id as id,
    email,
    "auth"."user"."auth_type" as auth_type,
    created_at,
    updated_at
  from
    "auth"."basic_user"
    inner join "auth"."user" on "auth"."user"."id" = "auth"."basic_user"."user_id"
);