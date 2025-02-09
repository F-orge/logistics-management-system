-- Add up migration script here
alter table "storage"."file" enable row level security;

create policy "User can read own file" on "storage"."file" for select using (
  owner_id = (select (payload ->> 'id')::uuid from "pgjwt"."verify"(current_setting('request.jwt'), current_setting('app.jwt_secret'))
));

create policy "User can remove own file" on "storage"."file" for delete using (
  owner_id = (select (payload ->> 'id')::uuid from "pgjwt"."verify"(current_setting('request.jwt'), current_setting('app.jwt_secret'))
));