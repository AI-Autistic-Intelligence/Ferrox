import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'Rust-YALC',
  tagline: 'The Ultimate Rust Arsenal',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://yalc.io',
  baseUrl: '/',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  // EREDITARIETÀ DEL TEMA (STATIC ASSETS)
  // Qui includiamo la cartella static del submodule aziendale.
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
          // EREDITARIETÀ DEL TEMA (CSS)
          // Puntiamo al CSS custom del submodule aziendale!
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
      // Usiamo il titolo del framework e il logo ereditato dal tema
      title: 'Rust-YALC',
      logo: {
        alt: 'Rust-YALC Logo',
        src: 'img/logo.jpg', 
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Documentation',
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
              to: '/docs/getting-started/introduction',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'Autistic Intelligence GitHub',
              href: 'https://github.com/AI-Autistic-Intelligence',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} AI-Autistic-Intelligence. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
