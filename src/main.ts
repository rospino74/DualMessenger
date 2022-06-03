import App from './Pages/App.svelte';
import { invoke } from '@tauri-apps/api/tauri'

const app = new App({
	target: document.getElementById("app"),
});

invoke("get_sys_version").then(version => {
	alert(version);
});

export default app;