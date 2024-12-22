import { defineConfig } from "rspress/config";

export default defineConfig({
  root: "docs",
  title: "Thesis - Documentation",
  description: "Documentation for the Thesis project",
  lang: "en",
  themeConfig: {
    sidebar: {
      "overview": [
        {
          text: "Overview",
          link: "/overview",
        },
      ],
    },
  },
});
