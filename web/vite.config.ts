import react from '@vitejs/plugin-react-swc'
import { presetAttributify, presetUno } from 'unocss'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [
    react(),
    UnoCSS({
      presets: [
        presetUno(),
        presetAttributify(),
        // presetIcons()
      ],
      rules: [

      ],
    }),
  ],
})
