import { type Kysely, sql } from "kysely";
import type { DB } from "kysely-codegen";
import createUser from "../queries/users/create";
import { faker } from "@faker-js/faker";

export default async function (db: Kysely<DB>): Promise<void> {
	const queries = [];
	for (let i = 0; i < 10000; i++) {
		const promise = db.executeQuery(createUser(db, {
			email: faker.internet.email(),
			password: faker.internet.password(),
			role: faker.helpers.arrayElement([
				"super_admin",
				"admin",
				"employee",
				"client",
			]),
		}));
		queries.push(promise);
	}
	await Promise.all(queries);
}
