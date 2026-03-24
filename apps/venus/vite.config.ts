import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import path from "path"
import { defineConfig } from "vite"

const host = process.env.TAURI_DEV_HOST;

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
    allowedHosts: ["cashier.sakaloka.id", "56.cashier.sakaloka.id"],
    host: true,
    port: 3000,
  },

  // Env variables starting with TAURI_ are exposed to the frontend
  envPrefix: ["VITE_", "TAURI_ENV_*"],
})
