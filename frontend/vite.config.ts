import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  build: {
    lib: {
      entry: 'src/index.tsx',
      formats: ['es'],
      fileName: 'index',
    },
    outDir: '.',
    emptyOutDir: false,
    rollupOptions: {
      external: [
        'react',
        'react/jsx-runtime',
        'react-dom',
        'lucide-react',
        'date-fns',
        /^\.\/store$/,
        /^\.\.\/store$/,
        /^\.\/utils\/format$/,
        /^\.\.\/utils\/format$/,
        /^\.\/utils\/toast$/,
        /^\.\.\/utils\/toast$/,
        /^\.\/components\/Common\/(Button|Card)$/,
        /^\.\/Common\/(Button|Card)$/,
      ],
      output: {
        format: 'es',
      },
    },
  },
});
