import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'Rust-FERROX',
  tagline: 'The Ultimate Rust Arsenal',
  favicon: 'img/favicon.png',

  // Set the production url of your site here
  url: 'https://ferrox.dev',
  baseUrl: '/',

  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'it', 'es', 'fr', 'de', 'zh', 'ja', 'ko', 'pt-BR'],
    localeConfigs: {
      en: { label: 'English', direction: 'ltr' },
      it: { label: 'Italiano', direction: 'ltr' },
      es: { label: 'Español', direction: 'ltr' },
      zh: { label: '中文 (Simplified)', direction: 'ltr' },
    },
  },

  // THEME INHERITANCE (STATIC ASSETS)
  // We include the static folder of the enterprise submodule here.
  staticDirectories: ['static', 'autistic-theme/static'],

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
        },
        blog: false,
        theme: {
          // THEME INHERITANCE (CSS)
          // We point to the custom CSS of the enterprise submodule!
          customCss: './autistic-theme/src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/docusaurus-social-card.jpg',
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      // We use the framework title and the logo inherited from the theme
      title: 'Rust-FERROX',
      logo: {
        alt: 'Rust-FERROX Logo',
        src: 'img/logo.jpg', 
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'ferroxSidebar',
          position: 'left',
          label: 'Documentation',
        },
        {
          href: 'https://discord.gg/Bx3CzGec7d',
          label: 'Discord',
          position: 'right',
        },
        {
          href: 'https://www.reddit.com/r/Ferrox/',
          label: 'Reddit',
          position: 'right',
        },
        {
          href: 'https://github.com/AI-Autistic-Intelligence',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Getting Started',
              to: '/docs/overview/introduction',
            },
          ],
        },
        {
          title: 'Community & Support',
          items: [
            {
              label: 'Official Email Support',
              href: 'mailto:info@ferrox-rust.dev',
            },
            {
              label: 'Discord Community Server',
              href: 'https://discord.gg/Bx3CzGec7d',
            },
            {
              label: 'Reddit Subreddit (r/Ferrox)',
              href: 'https://www.reddit.com/r/Ferrox/',
            },
            {
              label: 'Autistic Intelligence GitHub',
              href: 'https://github.com/AI-Autistic-Intelligence',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} AI-Autistic-Intelligence & Ferrox Contributors. Dual-licensed under MIT or Apache 2.0.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
