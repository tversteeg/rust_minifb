init();
async function init() {
	const [{setup_window, default: init}] = await Promise.all([
		import("../pkg/minifb_wasm.js"),
	]);

	await init();
	document.getElementById("status").innerText = "WebAssembly loaded!";

	setup_window();
}
