import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import path from "path"
import { defineConfig } from "vite"

const host = process.env.TAURI_DEV_HOST

function manualChunks(id: string): string | undefined {
  if (!id.includes("node_modules")) {
    return undefined
  }

  // Only split chunks that have zero React dependencies
  if (id.includes("/react-icons/")) {
    return "vendor-react-icons"
  }

  if (
    id.includes("/axios/") ||
    id.includes("/query-string/") ||
    id.includes("/dayjs/") ||
    id.includes("/lodash/") ||
    id.includes("/zod/")
  ) {
    return "vendor-utils"
  }

  if (id.includes("/@tauri-apps/")) {
    return "vendor-tauri"
  }

  return undefined
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  // Prevent Vite from obscuring Rust errors
  clearScreen: false,

  server: {
    // Tauri expects a fixed port; fail if it's already in use
    port: 5173,
    strictPort: true,
    // If Tauri sets TAURI_DEV_HOST, use it for mobile HMR
    host: host || false,
    hmr: host
      ? { protocol: "ws", host, port: 5174 }
      : undefined,
  },

  preview: {
    allowedHosts: ["pos.sakaloka.id", "dev-pos.sakaloka.id", "sit-pos.sakaloka.id", "demo-pos.sakaloka.id"],
    host: true,
    port: 3000,
  },

  // Env variables starting with TAURI_ are exposed to the frontend
  envPrefix: ["VITE_", "TAURI_ENV_*"],

  build: {
    rollupOptions: {
      output: {
        manualChunks,
      },
    },
  },
})
