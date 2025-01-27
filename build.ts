async function main() {
	const glob = new Bun.Glob("src/views/**/*.{ts}");
	const entrypoints = await Array.fromAsync(glob.scan("."));
	const result = await Bun.build({
		entrypoints,
		outdir: "src/views/assets",
		experimentalCss: true,
	});

	const fonts = new Bun.Glob(
		"node_modules/lucide-static/font/*.{ttf,woff,woff2}",
	);
	const fontFiles = await Array.fromAsync(fonts.scan("."));

	for (const fontFile of fontFiles) {
		const fileName = fontFile.split("/").pop();
		const file = Bun.file(fontFile);
		await Bun.write(`src/views/assets/${fileName}`, await file.text());
	}

	if (!result.success) {
		console.error(result.outputs);
	} else {
		console.log("Compiled files");
		console.table(entrypoints);
	}
}

main();
