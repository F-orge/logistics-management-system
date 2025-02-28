import { Kysely, PostgresDialect } from "kysely";
import { Pool } from "pg";
import { DB } from "kysely-codegen";

const db = new Kysely<DB>({
  dialect: new PostgresDialect({
    pool: new Pool({
      connectionString: process.env.DATABASE_URL,
    }),
  }),
});

// admin user
try {
  await db.insertInto("logistics.users").values({
    email: "admin@email.com",
    password: "RandomPassword1!",
    auth_type: "basic_auth",
  }).execute();
} catch (e) {
  console.error(e);
}

// another admin user
try {
  await db.insertInto("logistics.users").values({
    email: "admin2@email.com",
    password: "RandomPassword1!",
    auth_type: "basic_auth",
  }).execute();
} catch (e) {
  console.error(e);
}
