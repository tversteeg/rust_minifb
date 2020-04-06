init();

async function init() {
	const [{Chart, default: init}, {main, setup}] = await Promise.all([
		import("../pkg/minifb_wasm.js"),
		import("./index.js"),
	]);
	await init();
	setup(Chart);
	main();
}
