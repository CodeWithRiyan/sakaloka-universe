import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import path from "path"
import { defineConfig } from "vite"

const host = process.env.TAURI_DEV_HOST

function manualChunks(id: string): string | undefined {
  if (!id.includes("node_modules")) {
    return undefined
  }

  if (
    id.includes("/react/") ||
    id.includes("/react-dom/") ||
    id.includes("/scheduler/")
  ) {
    return "vendor-react"
  }

  if (
    id.includes("/react-router/") ||
    id.includes("/@reduxjs/") ||
    id.includes("/react-redux/") ||
    id.includes("/redux/")
  ) {
    return "vendor-router-state"
  }

  if (
    id.includes("/@radix-ui/") ||
    id.includes("/class-variance-authority/") ||
    id.includes("/clsx/") ||
    id.includes("/tailwind-merge/") ||
    id.includes("/sonner/") ||
    id.includes("/react-helmet") ||
    id.includes("/react-helmet-async/")
  ) {
    return "vendor-ui"
  }

  if (id.includes("/recharts/") || id.includes("/@tanstack/")) {
    return "vendor-data-viz"
  }

  if (
    id.includes("/swiper/") ||
    id.includes("/embla-carousel") ||
    id.includes("/motion/") ||
    id.includes("/canvas-confetti/")
  ) {
    return "vendor-motion"
  }

  if (id.includes("/react-icons/")) {
    return "vendor-react-icons"
  }

  if (
    id.includes("/axios/") ||
    id.includes("/query-string/") ||
    id.includes("/dayjs/") ||
    id.includes("/lodash/") ||
    id.includes("/zod/") ||
    id.includes("/react-hook-form/") ||
    id.includes("/@hookform/")
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
