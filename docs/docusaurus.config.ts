import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'Rust-FERROX',
  tagline: 'The Ultimate Rust Arsenal',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://ferrox.io',
  baseUrl: '/',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
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
