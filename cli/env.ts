import { Command } from "commander";
import { appendFile } from "node:fs/promises";

export const env = new Command("env")
  .addCommand(
    new Command("init")
      .description("Initialize the environment")
      .option("-f, --force", "Force initialization", false)
      .option("-e, --env <env>", "Environment to initialize", "development")
      .option(
        "-p, --path <path>",
        "Path to the environment file",
        ".env.development",
      )
      .action((options: {
        force: boolean;
        env: string;
        path: string;
      }) => {
        console.log("Initializing environment", options);
      }),
  )
  .addCommand(
    new Command("add")
      .description("Add a new environment variable")
      .requiredOption("-n, --name <name>", "Name of the environment variable")
      .requiredOption(
        "-v, --value <value>",
        "Value of the environment variable",
      )
      .action(async (options: {
        name: string;
        value: string;
      }) => {
        const { name, value } = options;
        await appendFile(".env.development", `${name}=${value}\n`);
        console.log(`Environment variable ${name} added`);
      }),
  )
  .addCommand(
    new Command("remove")
      .description("Remove an environment variable")
      .requiredOption("-n, --name <name>", "Name of the environment variable")
      .option(
        "-e, --env <env>",
        "Environment to remove the variable from",
        "development",
      )
      .action(async (options: { name: string; env: string }) => {
        const { name, env } = options;

        const filePath = `.env.${env}`;
        const file = Bun.file(filePath);
        const content = await file.text();
        const lines = content.split("\n");
        const filteredLines = lines.filter((line) =>
          !line.startsWith(`${name}=`)
        );
        await Bun.write(filePath, filteredLines.join("\n"));

        console.log(`Removed environment variable ${name} from .env.${env}`);
      }),
  )
  .addCommand(
    new Command("list")
      .description("List all environment variables")
      .option(
        "-e, --env <env>",
        "Environment to list variables from",
        "development",
      )
      .action(async (options: { env: string }) => {
        const { env } = options;
        const filePath = `.env.${env}`;
        const file = Bun.file(filePath);
        const content = await file.text();
        const lines = content.split("\n");
        const variables = lines.map((line) => {
          const [name, value] = line.split("=");
          if (name.startsWith("PUBLIC_")) {
            return { name: name, value };
          }
          return { name, value: "********" };
        });
        console.table(variables);
      }),
  );
