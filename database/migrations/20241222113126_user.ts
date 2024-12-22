import { type CompiledQuery, type Kysely, sql } from "kysely";

export function up(db: Kysely<any>): CompiledQuery[] {
	// Write your migration here
	return [
		db.schema.createSchema("auth").compile(),
		db.schema
			.withSchema("auth")
			.createTable("user")
			.addColumn(
				"id",
				"uuid",
				(col) => col.primaryKey().notNull().defaultTo(sql`gen_random_uuid()`),
			)
			.addColumn("email", "varchar(255)", (col) => col.notNull().unique())
			.addColumn("password", "varchar(128)", (col) => col.notNull())
			.addColumn("role", "varchar(255)", (col) =>
				col
					.notNull()
					.check(sql`role IN ('super_admin', 'admin', 'employee', 'client')`))
			.addColumn(
				"created_at",
				"timestamp",
				(col) => col.notNull().defaultTo(sql`current_timestamp`),
			)
			.addColumn(
				"updated_at",
				"timestamp",
				(col) => col.notNull().defaultTo(sql`current_timestamp`),
			)
			.compile(),
	];
}

export function down(db: Kysely<any>): CompiledQuery[] {
	// Write your rollback here
	return [db.schema.dropSchema("auth").ifExists().cascade().compile()];
}
