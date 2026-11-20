import { defineConfig } from 'oxfmt'

export default defineConfig({
  semi: false,
  singleQuote: true,
  sortImports: {
    internalPattern: ['~/', '@/', '#'],
    newlinesBetween: false,
    sortSideEffects: true,
  },
  sortTailwindcss: {
    stylesheet: './src/routes/layout.css',
    functions: ['clsx', 'cn', 'tv'],
  },
  svelte: true,
})
