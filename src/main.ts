import App from './Pages/App.svelte';
import { invoke } from '@tauri-apps/api/tauri'

const app = new App({
	target: document.getElementById("app"),
});

invoke("get_sys_version").then(version => {
	const el = document.createElement("div");
	el.innerHTML = `<h1>Tauri-Apps v${version}</h1>`;
	document.body.appendChild(el);
});

export default app;