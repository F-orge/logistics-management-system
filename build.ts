async function main() {
	const glob = new Bun.Glob("src/views/**/*.{ts}");
	const entrypoints = await Array.fromAsync(glob.scan("."));
	const result = await Bun.build({
		entrypoints,
		outdir: "dist",
		experimentalCss: true,
	});
}

main();
