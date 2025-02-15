-- Add up migration script here
do $$
declare
  _super_admin_token varchar;
  _manager_token varchar;
  _employee_token varchar;
begin

  -- setup
  set role nextjs;

  -- tests

  -- clean
  set role postgres;
  perform 'rollback';
exception
  when others then
    perform 'rollback';
    raise;  
end $$ language plpgsql;