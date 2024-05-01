import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    minify: false,
  },
  resolve: {
    alias: {
      "@compiled": "/src/compiled",
    },
  },
})