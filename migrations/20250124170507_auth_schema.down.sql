-- Add down migration script here
drop schema "auth" cascade;

drop extension pgjwt;

drop extension pgcrypto;

drop user web;