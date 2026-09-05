import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  ferroxSidebar: [
    {
      type: 'category',
      label: '🚀 Overview',
      items: [
        'overview/introduction',
        'overview/lifecycle',
      ],
    },
    {
      type: 'category',
      label: '🎓 Tutorial: Mini-Project',
      items: [
        'tutorial/01-setup',
        'tutorial/02-building-the-core',
        'tutorial/03-code-factory',
      ],
    },
    {
      type: 'category',
      label: '🏗️ Fundamentals',
      items: [
        'fundamentals/controllers',
        'fundamentals/providers',
        'fundamentals/middlewares',
      ],
    },
    {
      type: 'category',
      label: '🛡️ Architectural Abstractions',
      items: [
        'abstractions/pipes',
        'abstractions/guards',
      ],
    },
    {
      type: 'category',
      label: '🔒 Enterprise Security',
      items: [
        'security/jwt',
      ],
    },
    {
      type: 'category',
      label: '🧠 Complex Architectures',
      items: [
        'architectures/cqrs',
        'architectures/caching',
        'architectures/sagas',
      ],
    },
    {
      type: 'category',
      label: '🛠️ CLI & Code Factory',
      items: [
        'cli/code-factory',
      ],
    }
  ],
};

export default sidebars;
