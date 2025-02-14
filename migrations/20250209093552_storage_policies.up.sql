-- Add up migration script here
alter table "storage"."file" enable row level security;
alter table "storage"."file_access" enable row level security;

create policy "registered users can insert file" on "storage"."file" for insert to nextjs with check (
  exists (select 1 from "auth"."basic_user_view" where id = ("auth"."current_user"()::json->>'id')::uuid)
);

create policy "user can access shared files" on "storage"."file" for select to nextjs using (
  exists (select 1 from "storage"."file_access" where "file_access"."file_id" = "file"."id" and "user_id" = ("auth"."current_user"()::json->>'id')::uuid) or "is_public" = true
);

create policy "user can update their own files" on "storage"."file" for update to nextjs using (
  owner_id = ("auth"."current_user"()::json->>'id')::uuid
);

create policy "user can delete their own file" on "storage"."file" for delete to nextjs using (
  owner_id = ("auth"."current_user"()::json->>'id')::uuid
);

create policy "registered users can only read the file" on "storage"."file_access" for insert to nextjs with check (
  true
);

create policy "registered users can access file permissions they owned or shared to" on "storage"."file_access" for select to nextjs using (
  true
);

create policy "registered users can update file access to their own file" on "storage"."file_access" for update to nextjs using (
  exists (select 1 from "storage"."file_access" where "user_id" = ("auth"."current_user"()::json->>'id')::uuid and file_id in (select id from "storage"."file" where owner_id = ("auth"."current_user"()::json->>'id')::uuid))
);