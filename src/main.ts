import { initializeI18n } from "./i18n";
import App from "./Pages/App.svelte";

// Initialize the i18n library
await initializeI18n();

// Create the app component
const app = new App({
	target: document.getElementById("app"),
});

export default app;