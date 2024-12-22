import { type CompiledQuery, type Kysely, sql } from "kysely";
import type { DB } from "kysely-codegen";

export default function (db: Kysely<DB>, options: {}): CompiledQuery {
	return db.selectFrom("auth.user").selectAll().where(
		"auth.user.email",
		"=",
		"",
	).compile();
}
