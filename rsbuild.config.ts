import { defineConfig } from "@rsbuild/core";
import { pluginBabel } from "@rsbuild/plugin-babel";
import { pluginSolid } from "@rsbuild/plugin-solid";
import tailwindcss from "tailwindcss";
import path from "node:path";
export default defineConfig({
  plugins: [
    pluginSolid(),
    pluginBabel({
      include: /\.(?:jsx|tsx)$/,
    }),
  ],
  source: {
    entry: {
      index: "./src/frontend/entry-client.tsx",
    },
  },
  tools: {
    postcss: {
      postcssOptions: {
        plugins: [
          tailwindcss(),
        ],
      },
    },
  },
  resolve: {
    alias: {
      "~": path.resolve(__dirname, "./src"),
    },
  },
});
