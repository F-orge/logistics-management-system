#!/usr/bin/env bun

import { program } from "commander";
import { dev } from "./dev";
import { env } from "./env";

program.addCommand(dev);
program.addCommand(env);

program.parse(process.argv);
