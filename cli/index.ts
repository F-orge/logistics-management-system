#!/usr/bin/env bun

import { program } from "commander";
import { dev } from "./dev";
import { env } from "./env";
import { database } from "./database";

program.addCommand(dev);
program.addCommand(env);
program.addCommand(database);

program.parse(process.argv);
