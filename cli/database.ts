import { Command } from "commander";
import { type CompiledQuery, Kysely, PostgresDialect } from "kysely";
import type { DB } from "kysely-codegen";
import pg from "pg";

export const database = new Command("database");

database
  .command("seed")
  .description("Seed the database with some data")
  .addCommand(
    new Command("list").description("List all the seeds").action(() => {
      console.log("Listing seeds");
    }),
  )
  .addCommand(
    new Command("run").description("Run the seeds").action(() => {
      console.log("Running seeds");
    }),
  )
  .addCommand(
    new Command("add").description("Add a new seed file").action(() => {
      console.log("Adding a new seed file");
    }),
  );

database
  .command("migrate")
  .description("Migrate the database")
  .addCommand(
    new Command("info")
      .description("List all avaiable migrations")
      .option(
        "-s, --source <source>",
        "Source of the migrations",
        "database/migrations/sql",
      )
      .option(
        "--database-url <databaseUrl>",
        "Database URL",
        process.env.DATABASE_URL,
      )
      .action(
        async ({
          source,
          databaseUrl,
        }: { source: string; databaseUrl: string }) => {
          await Bun
            .$`sqlx migrate info --database-url ${databaseUrl} --source=${source}`;
        },
      ),
  )
  .addCommand(
    new Command("add")
      .description("Add a new migration")
      .requiredOption("-n, --name <name>", "Name of the migration")
      .option(
        "-s, --source <source>",
        "Source of the migrations",
        "database/migrations",
      )
      .action(
        async ({
          name,
          source,
        }: {
          name: string;
          source: string;
        }) => {
          const fileName = `${source}/${
            new Date()
              .toISOString()
              .replace(/[-:.TZ]/g, "")
              .slice(0, 14)
          }_${name}.ts`;

          const content = `
          import { type Kysely, type CompiledQuery, sql } from "kysely";
          
          export function up(db: Kysely<any>):CompiledQuery[] {
            // Write your migration here
            return [
              db.schema
                .createTable("dummy_table")
                .addColumn(
                  "id",
                  "uuid",
                  (col) => col.primaryKey().notNull().defaultTo(sql\`gen_random_uuid()\`),
                )
                .addColumn(
                  "created_at",
                  "timestamp",
                  (col) => col.notNull().defaultTo(sql\`current_timestamp\`),
                )
                .addColumn(
                  "updated_at",
                  "timestamp",
                  (col) => col.notNull().defaultTo(sql\`current_timestamp\`),
                ).compile()
            ]
          }

          export function down(db: Kysely<any>):CompiledQuery[] {
            // Write your rollback here
            return [
              db.schema.dropTable("dummy_table").cascade().compile()
            ]
          }
        `;
          Bun.write(fileName, content);
          await Bun.$`bunx biome format ${fileName} --fix`.quiet();
          console.log(`Adding migration ${fileName}`);
        },
      ),
  )
  .addCommand(
    new Command("run")
      .description("Run all pending migrations")
      .option(
        "-s, --source <source>",
        "Source of the migrations",
        "database/migrations",
      )
      .option("-c, --compile", "Compile the migrations before running", false)
      .option(
        "--database-url <databaseUrl>",
        "Database URL",
        process.env.DATABASE_URL,
      )
      .option("--env-file <envFile>", "Environment file", ".env.development")
      .action(
        async ({
          source,
          compile,
          databaseUrl,
          envFile,
        }: {
          source: string;
          compile: boolean;
          databaseUrl: string;
          envFile: string;
        }) => {
          if (compile) {
            await Bun
              .$`bun run cli/index.ts database migrate compile --source ${source}`;
          }
          await Bun
            .$`sqlx migrate run --database-url ${databaseUrl} --source ${source}/sql`;
          await Bun
            .$`bun kysely-codegen --dialect postgres --env-file ${envFile}`;
        },
      ),
  )
  .addCommand(
    new Command("rollback")
      .description("Rollback the last migration")
      .option(
        "-s, --source <source>",
        "Source of the migrations",
        "database/migrations",
      )
      .option("-c, --compile", "Compile the migrations before running", false)
      .option(
        "--database-url <databaseUrl>",
        "Database URL",
        process.env.DATABASE_URL,
      )
      .action(
        async ({
          source,
          databaseUrl,
          compile,
        }: {
          source: string;
          databaseUrl: string;
          compile: boolean;
        }) => {
          if (compile) {
            await Bun
              .$`bun run cli/index.ts database migrate compile --source ${source}`
              .quiet();
          }
          await Bun
            .$`sqlx migrate revert --database-url ${databaseUrl} --source ${source}/sql`;
          await Bun.$`bun kysely-codegen`;
        },
      ),
  )
  .addCommand(
    new Command("compile")
      .description("Compile all migrations to a .sql file")
      .option(
        "-s, --source <source>",
        "Source of the migrations",
        "database/migrations",
      )
      .option(
        "--database-url <databaseUrl>",
        "Database URL",
        process.env.DATABASE_URL,
      )
      .option(
        "--output <output>",
        "Output directory",
        "database/migrations/sql",
      )
      .action(
        async ({
          source,
          databaseUrl,
          output,
        }: {
          source: string;
          databaseUrl: string;
          output: string;
        }) => {
          const dialect = new PostgresDialect({
            pool: new pg.Pool({
              connectionString: databaseUrl,
            }),
          });

          const db = new Kysely({ dialect });

          // Get all migrations
          const files = new Bun.Glob("**/*.ts");
          const completed = [];

          for await (const file of files.scan(source)) {
            const {
              up,
              down,
            }: {
              // biome-ignore lint/suspicious/noExplicitAny: allow any type for migrations
              up: (db: Kysely<any>) => CompiledQuery[];
              // biome-ignore lint/suspicious/noExplicitAny: allow any type for migrations
              down: (db: Kysely<any>) => CompiledQuery[];
            } = await import(`${source}/${file}`);
            const compiledUp = up(db).map((query) => query.sql);
            const compiledDown = down(db).map((query) => query.sql);

            // create a .sql file for each migration
            const upFileName = file.replace(".ts", ".up.sql");
            const downFileName = file.replace(".ts", ".down.sql");

            const comment = "-- DO NOT EDIT. AUTO GENERATED --\n\n";

            await Bun.write(
              `${output}/${upFileName}`,
              `${comment}${compiledUp.join("\n\n")}`,
            );
            await Bun.write(
              `${output}/${downFileName}`,
              `${comment}${compiledDown.join("\n\n")}`,
            );
            completed.push({
              file,
              up: upFileName,
              down: downFileName,
            });
          }
          console.log("Migrations compiled");
          console.table(completed);

          await db.destroy();
        },
      ),
  );

database
  .command("query")
  .description("Query management")
  .addCommand(
    new Command("list").description("List all queries")
      .option(
        "-s, --source <source>",
        "Source of the queries",
        "database/queries",
      )
      .action(async () => {
        const tsFiles = new Bun.Glob("**/*.ts");
        const sqlFiles = new Bun.Glob("**/*.sql");
        const queries: {
          definition: string;
          compiled: string;
        }[] = [];

        for await (const file of tsFiles.scan("database/queries")) {
          if (
            !await Bun.file(
              `database/queries/compiled/${file.replace(".ts", ".sql")}`,
            ).exists()
          ) {
            continue;
          }
          queries.push({
            definition: file,
            compiled: file.replace(".ts", ".sql"),
          });
        }

        console.table(queries);
      }),
  )
  .addCommand(
    new Command("add")
      .description("Add a new query")
      .requiredOption("-n, --name <name>", "Name of the query")
      .action(async ({ name }: { name: string }) => {
        const fileName = `database/queries/${name}.ts`;
        const content = `
          import { type Kysely, type CompiledQuery, sql } from "kysely";
          import type { DB } from "kysely-codegen";

          export default function(db: Kysely<DB>,options:{}):CompiledQuery {
            return sql\`\`.compile(db);
          };
        `;
        Bun.write(fileName, content);
        await Bun.$`bunx biome format ${fileName} --fix`.quiet();
        console.log(`Adding query ${fileName}`);
      }),
  )
  .addCommand(
    new Command("compile")
      .description("Compile all queries to a .sql file")
      .option(
        "-s, --source <source>",
        "Source of the migrations",
        "database/queries",
      )
      .option(
        "--database-url <databaseUrl>",
        "Database URL",
        process.env.DATABASE_URL,
      )
      .option(
        "--output <output>",
        "Output directory",
        "database/queries/compiled",
      )
      .action(
        async ({
          source,
          databaseUrl,
          output,
        }: {
          source: string;
          databaseUrl: string;
          output: string;
        }) => {
          const dialect = new PostgresDialect({
            pool: new pg.Pool({
              connectionString: databaseUrl,
            }),
          });

          const db = new Kysely<DB>({ dialect });

          // Get all queries
          const files = new Bun.Glob("**/*.ts");

          for await (const file of files.scan(source)) {
            const query: (db: Kysely<DB>, options: {}) => CompiledQuery = (
              await import(`${source}/${file}`)
            ).default;
            const compiled = query(db, {}).sql;

            // create a .sql file for each query
            const comment =
              `-- DO NOT EDIT. AUTO GENERATED. File name: ${file} --\n\n`;
            const fileName = file.replace(".ts", ".sql");

            Bun.write(`${output}/${fileName}`, `${comment}${compiled}`);
          }
        },
      ),
  )
  .addCommand(
    new Command("run").description("Run a query").action(() => {
      console.log("Running a query");
    }),
  );
