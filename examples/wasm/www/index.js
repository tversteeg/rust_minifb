class Window {}

const status = document.getElementById("status");

/** Main entry point */
export function main() {
	status.innerText = "WebAssembly loaded!"
}

export function setup(WasmWindow) {
	Window = WasmWindow;
}
