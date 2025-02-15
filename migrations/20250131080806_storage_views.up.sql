/* plpgsql-language-server:disable validation */
-- Add up migration script here
create view
  "storage"."file_view" as (
    select
      "storage"."file".id,
      "storage"."file".name,
      "storage"."file".type,
      "storage"."file".size,
      "storage"."file".owner_id
    from
      "storage"."file"
  );

grant
select
  on "storage"."file_view" to nextjs;