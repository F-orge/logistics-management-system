import { defineConfig } from "@rsbuild/core";
import { pluginBabel } from "@rsbuild/plugin-babel";
import { pluginSolid } from "@rsbuild/plugin-solid";
import { pluginImageCompress } from "@rsbuild/plugin-image-compress";

import tailwindcss from "@tailwindcss/postcss";
import path from "node:path";

export default defineConfig({
	tools: {
		postcss: {
			postcssOptions: {
				plugins: [tailwindcss()],
			},
		},
	},
	source: {
		entry: {
			index: "./src/views/index.ts",
		},
	},
	output: {
		minify: true,
		distPath: {
			root: "./src/views/assets",
		},
	},
	dev: {
		writeToDisk: true,
	},
});
