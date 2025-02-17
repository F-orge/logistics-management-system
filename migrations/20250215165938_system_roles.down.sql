-- Add down migration script here
set role postgres;

-- internal system roles
drop user developer;

drop user migration;

drop role anon;

drop extension pgjwt;

drop extension pgcrypto;

reassign owned by web to postgres;

drop owned by web;

drop role web;

