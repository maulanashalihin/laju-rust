import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import inertia from '@inertiajs/vite'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [
    tailwindcss(),
    svelte(),
    inertia(),
  ],
  build: {
    outDir: 'dist',
    manifest: true,
  },
  server: {
    port: 5173,
    strictPort: true,
    cors: {
      origin: ['http://0.0.0.0:3000', 'http://localhost:3000'],
      credentials: true,
    },
  },
})
