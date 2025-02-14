-- Add up migration script here
alter table "auth"."basic_user" enable row level security;

create policy "nextjs can create users" on "auth"."basic_user" for insert to nextjs with check (true);

create policy "nextjs can update users" on "auth"."basic_user" for update to nextjs using (true);

create policy "nextjs can delete users" on "auth"."basic_user" for delete to nextjs using (true);

create policy "nextjs can read users" on "auth"."basic_user" for select to nextjs using (true);

grant select on "auth"."basic_user_view" to nextjs; 