import { fileURLToPath } from 'node:url';

import { defineConfig } from '@rspress/core';

export default defineConfig({
  root: 'docs',
  title: 'LoL Wiki',
  description: 'Generated League Wiki champions, items, and runes documentation.',
  lang: 'en',
  logoText: 'LoL Wiki',
  outDir: 'out',
  globalStyles: fileURLToPath(new URL('./styles/global.css', import.meta.url)),
  route: {
    extensions: ['.md', '.mdx'],
  },
  search: {
    codeBlocks: false,
  },
  markdown: {
    link: {
      checkDeadLinks: false,
    },
    image: {
      checkDeadImages: false,
    },
  },
  ssg: false,
  themeConfig: {
    enableScrollToTop: true,
    footer: {
      message: 'Generated from League Wiki exports and the Rust converter.',
    },
  },
});