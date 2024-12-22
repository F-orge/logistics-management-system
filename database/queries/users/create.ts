import { type CompiledQuery, type Kysely, sql } from "kysely";
import type { DB } from "kysely-codegen";

export default function (db: Kysely<DB>, options: {
	email: string;
	password: string;
	role: string;
}): CompiledQuery {
	return db.insertInto("auth.user").values({
		email: options.email,
		password: options.password,
		role: options.role,
	}).compile();
}
