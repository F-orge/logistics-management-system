async function main() {
	const glob = new Bun.Glob("src/views/**/*.{ts}");
	const entrypoints = await Array.fromAsync(glob.scan("."));
	const result = await Bun.build({
		entrypoints,
		outdir: "src/views/assets",
		experimentalCss: true,
	});
	if (!result.success) {
		console.error(result.outputs);
	} else {
		console.log("Compiled files");
		console.table(entrypoints);
	}
}

main();
