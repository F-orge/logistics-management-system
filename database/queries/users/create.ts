import { type CompiledQuery, type Kysely, sql } from "kysely";
import type { DB } from "kysely-codegen";

export default function (db: Kysely<DB>, options: {}): CompiledQuery {
	return db.insertInto("auth.user").values({
		email: "",
		password: "",
		role: "",
	}).compile();
}
