import { defineConfig } from "vite"
import sveltePreprocess from "svelte-preprocess"
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { join } from "path"

const style_folder = join(__dirname, "./src/styles");

export default defineConfig({
  root: "src",
  publicDir: "../dist",
  optimizeDeps: {
    exclude: ["svelte-navigator"], // https://github.com/mefechoel/svelte-navigator#im-using-vite-why-am-i-getting-errors-with-svelte-navigator
  },
  envPrefix: ['VITE_', 'TAURI_'], // https://tauri.app/v1/guides/getting-started/setup/vite
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
      compilerOptions: {
        dev: process.env.NODE_ENV !== "production",
        cssHash: ({ hash, css, name, filename }) => `${name.toLowerCase()}-${hash(css)}${hash(filename)}`,
      }
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
    // ---- https://tauri.app/v1/guides/getting-started/setup/vite
    minify: !process.env.TAURI_DEBUG && "esbuild",
    sourcemap: !!process.env.TAURI_DEBUG,
    // ----
    target: "es2022",
    outDir: "../dist",
    emptyOutDir: true,
    rollupOptions: {
      // I like that :) https://stackoverflow.com/q/71180561/
      output: {
        assetFileNames: (assetInfo) => {
          let extType = assetInfo?.name?.split('.')[1] ?? "assets";
          if (/png|jpe?g|svg|gif|tiff|bmp|ico/i.test(extType)) {
            extType = "img";
          } else if (/woff|woff2/.test(extType)) {
            extType = "css";
          }
          return `${extType}/[name]-[hash][extname]`;
        },
        chunkFileNames: "js/[name]-[hash].js",
        entryFileNames: "js/[name]-[hash].js",
      },
    },
  },
});
