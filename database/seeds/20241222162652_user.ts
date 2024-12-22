import type { Kysely } from "kysely";
import type { DB } from "kysely-codegen";
import createUser from "../queries/users/create";

export default async function (db: Kysely<DB>): Promise<void> {
	await db.executeQuery(createUser(db, {
		email: "super.admin@example.com",
		password: "password",
		role: "super_admin",
	}));

	await db.executeQuery(createUser(db, {
		email: "admin@example.com",
		password: "password",
		role: "admin",
	}));

	await db.executeQuery(createUser(db, {
		email: "employee@example.com",
		password: "password",
		role: "employee",
	}));

	await db.executeQuery(createUser(db, {
		email: "client@example.com",
		password: "password",
		role: "client",
	}));

	console.log("Seeded users");
}
