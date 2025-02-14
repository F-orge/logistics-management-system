-- Add up migration script here
do $$
declare
  _sample_1_token varchar;
  _sample_2_token varchar;
begin 
  -- setup
  set role nextjs;
  perform set_config('app.jwt_secret','secret',true);

  -- actors
  insert into "auth"."basic_user"(email,password) values ('sample1@email.com','randompassword');
  insert into "auth"."basic_user"(email,password) values ('sample2@email.com','randompassword');

  -- tokens
  select "auth"."basic_login"('sample1@email.com','randompassword') into _sample_1_token;
  select "auth"."basic_login"('sample2@email.com','randompassword') into _sample_2_token;

  -- dummy data
  perform set_config('request.jwt',_sample_1_token,true);
  insert into "storage"."file"(name,type,size,owner_id,is_public) values ('sample1.txt','text/plain',5000,("auth"."current_user"()::json->>'id')::uuid,true);
  insert into "storage"."file"(name,type,size,owner_id) values ('sample1.pdf','application/pdf',5000,("auth"."current_user"()::json->>'id')::uuid);
  insert into "storage"."file"(name,type,size,owner_id) values ('sample1.md','application/markdown',5000,("auth"."current_user"()::json->>'id')::uuid);

  perform set_config('request.jwt',_sample_2_token,true);
  insert into "storage"."file"(name,type,size,owner_id) values ('sample2.txt','text/plain',5000,("auth"."current_user"()::json->>'id')::uuid);
  insert into "storage"."file"(name,type,size,owner_id) values ('sample2.pdf','application/pdf',5000,("auth"."current_user"()::json->>'id')::uuid);
  insert into "storage"."file"(name,type,size,owner_id) values ('sample2.md','application/markdown',5000,("auth"."current_user"()::json->>'id')::uuid);

  -- file access
  perform set_config('request.jwt',_sample_1_token,true); 
  -- policies: clients and users can access public file and its own file. sample 1 will have 3 files and sample 2 will have 4 files
  if (select count(*) from "storage"."file") != 3 then 
    raise exception 'file read policy error. must be 3';
  end if;
  
  perform set_config('request.jwt',_sample_2_token,true); 
  if (select count(*) from "storage"."file") != 4 then 
    raise exception 'file read policy error. must be 4';
  end if;

  perform set_config('request.jwt',_sample_1_token,true); 
  -- make sample1.pdf and sample1.md public
  update "storage"."file" set is_public = true where name in ('sample1.pdf','sample1.md');

  -- file access check
  -- sample 2 will now have 6 files
  perform set_config('request.jwt',_sample_2_token,true); 
  if (select count(*) from "storage"."file") != 6 then 
    raise exception 'file update policy error must be 6';
  end if;

  -- share sample2.pdf and sample2.md to sample1
  perform set_config('request.jwt',_sample_2_token,true);
  insert into "storage"."file_access"(file_id,user_id) values (
    (select id from "storage"."file" where name = 'sample2.pdf'),
    ((select payload from verify(_sample_1_token,current_setting('app.jwt_secret')))::json->>'id')::uuid
  );
  insert into "storage"."file_access"(file_id,user_id) values (
    (select id from "storage"."file" where name = 'sample2.md'),
    ((select payload from verify(_sample_1_token,current_setting('app.jwt_secret')))::json->>'id')::uuid
  );
  
  perform set_config('request.jwt',_sample_1_token,true);
  -- file access check
  -- sample 1 will now have 5 files
  if (select count(*) from "storage"."file") != 5 then 
    raise exception 'file share policy error. must be 5';
  end if;

  perform set_config('request.jwt',_sample_1_token,true);
  -- delete
  delete from "storage"."file" where "owner_id" = ("auth"."current_user"()::json->>'id')::uuid;

  if (select count(*) from "storage"."file") != 2 then
    raise exception 'file delete policy error. must be 2';
  end if;

  perform set_config('request.jwt',_sample_2_token,true);
  -- delete
  delete from "storage"."file" where "owner_id" = ("auth"."current_user"()::json->>'id')::uuid;

  if (select count(*) from "storage"."file") != 0 then
    raise exception 'file delete policy error. must be 0';
  end if;

  -- clean
  set role postgres;
  perform 'rollback';
exception
  when others then
    perform 'rollback';
    raise;
end $$ language plpgsql;