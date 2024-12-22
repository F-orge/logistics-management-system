import { Command } from "commander";

export const dev = new Command("dev")
	.addCommand(new Command("init"))
	.addCommand(new Command("start"));
