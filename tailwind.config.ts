const { addDynamicIconSelectors } = require("@iconify/tailwind");

export default {
	content: [
		"./src/**/*.{js,ts,templ,go}",
		"./node_modules/flyonui/dist/js/*.js",
	],
	flyonui: {
		themes: ["corporate"],
	},
	plugins: [
		require("flyonui"),
		require("flyonui/plugin"),
		addDynamicIconSelectors(),
	],
};
