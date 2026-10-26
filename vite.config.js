import { defineConfig } from "vite";

export default defineConfig({
  // Impedisce a vite di oscurare gli errori di Rust
  clearScreen: false,
  // Tauri utilizza una porta fissa
  server: {
    port: 1420,
    strictPort: true,
  },
  // Prefissi per le variabili d'ambiente di Tauri
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Target compatibile con Tauri
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});