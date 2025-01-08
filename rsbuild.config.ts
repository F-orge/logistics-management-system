import { defineConfig } from "@rsbuild/core";
import { pluginBabel } from "@rsbuild/plugin-babel";
import { pluginSolid } from "@rsbuild/plugin-solid";
import { pluginImageCompress } from "@rsbuild/plugin-image-compress";

import tailwindcss from "tailwindcss";
import path from "node:path";

export default defineConfig({
	plugins: [
		pluginSolid(),
		pluginBabel({
			include: /\.(?:jsx|tsx)$/,
		}),
		pluginImageCompress(),
	],
	source: {
		entry: {
			index: "./src/views/entry-client.tsx",
		},
	},
	tools: {
		postcss: {
			postcssOptions: {
				plugins: [tailwindcss()],
			},
		},
	},
	output: {
		minify: true,
		distPath: {
			root: "./target/release/frontend-build",
		},
	},
	html: {
		tags: [{
			tag: "script",
			attrs: {
				src:
					"https://unpkg.com/tailwindcss-intersect@2.x.x/dist/observer.min.js",
				defer: true,
			},
		}],
	},
	resolve: {
		alias: {
			"~": path.resolve(__dirname, "./src/frontend"),
		},
	},
	dev: {
		writeToDisk: true,
	},
});
