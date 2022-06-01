import { defineConfig } from "vite"
import { svelte } from "@sveltejs/vite-plugin-svelte"
import sveltePreprocess from "svelte-preprocess"
const path = require("path");

const style_folder = path.join(__dirname, './src/styles');

export default defineConfig({
  root: "src",
  publicDir: "../dist",
  plugins: [
    svelte({
      preprocess: sveltePreprocess({
        replace: [
          [/process\.env\.NODE_ENV/g, JSON.stringify(process.env.NODE_ENV)]
        ],
        postcss: true,
        scss: {
          includePaths: [style_folder],
        }
      }),
    })
  ],
  server: {
    port: 8080,
    watch: {
      ignored: ["src-tauri/**"],
      usePolling: true
    }
  },
  build: {
    target: "es2015",
    outDir: "../dist",
    emptyOutDir: true,
  }
})
