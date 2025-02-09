-- Add up migration script here
alter table "auth"."basic_user" enable row level security;

create policy "User can update own password" on "auth"."basic_user" for update using (
  user_id = (select (payload ->> 'id')::uuid from "pgjwt"."verify"(current_setting('request.jwt'), current_setting('app.jwt_secret')))
);

create policy "User can delete own account" on "auth"."basic_user" for delete using (
  user_id = (select (payload ->> 'id')::uuid from "pgjwt"."verify"(current_setting('request.jwt'), current_setting('app.jwt_secret'))
));

create policy "User can read own account" on "auth"."basic_user" for select using (
  user_id = (select (payload ->> 'id')::uuid from "pgjwt"."verify"(current_setting('request.jwt'), current_setting('app.jwt_secret'))
));