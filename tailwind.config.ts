import type { Config } from "tailwindcss";
import flyonui from "flyonui";
import flyonuiPlugin from "flyonui/plugin";

export default {
	darkMode: ["class"],
	content: [
		"./apps/**/*.{jinja,ts,js}",
		"./crates/**/*.{jinja,ts,js}",
		"./node_modules/flyonui/dist/js/*.js",
	],
	flyonui: {
		themes: [
			"corporate",
			{
				custom: {
					"primary": "#ff6900",
					"primary-content": "#dcd9ff",
					"secondary": "#3f3f46",
					"secondary-content": "#ffd8d1",
					"accent": "#ffd6a8",
					"accent-content": "#160000",
					"neutral": "#0a0609",
					"neutral-content": "#c7c5c6",
					"base-100": "#fff7ed",
					"base-200": "#d4ded9",
					"base-300": "#b5beba",
					"base-content": "#141615",
					"info": "#ffaf7a",
					"info-content": "#001616",
					"success": "#00d400",
					"success-content": "#001000",
					"warning": "#ff8400",
					"warning-content": "#160600",
					"error": "#f7305e",
					"error-content": "#150103",
				},
			},
		],
	},
	plugins: [require("tailwindcss-animate"), flyonui, flyonuiPlugin],
} satisfies Config;
