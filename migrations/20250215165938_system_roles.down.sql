-- Add down migration script here
set role postgres;

-- internal system roles
drop user developer;

drop user migration;

drop role web;

drop role anon;

drop extension pgjwt;

drop extension pgcrypto;

