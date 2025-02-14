-- Add up migration script here
alter table "storage"."file" enable row level security;

create policy "nextjs can insert file" on "storage"."file" for insert to nextjs with check (true);

create policy "nextjs can read public file" on "storage"."file" for select to nextjs using (
  is_public = true
);

create policy "user can access shared files" on "storage"."file" for select to nextjs using (
  exists (select 1 from "storage"."file_access" where "file_access".file_id = "file".id and "user_id" = ("auth"."current_user"()::json->>'id')::uuid)
);

create policy "user can update their own files" on "storage"."file" for update to nextjs using (
  owner_id = ("auth"."current_user"()::json->>'id')::uuid
);

create policy "user can delete their own file" on "storage"."file" for delete to nextjs using (
  owner_id = ("auth"."current_user"()::json->>'id')::uuid
);