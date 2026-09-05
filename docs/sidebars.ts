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
      items: ['overview/introduction'],
    },
    {
      type: 'category',
      label: '🏗️ Fundamentals',
      items: ['fundamentals/controllers'],
    },
    {
      type: 'category',
      label: '🛡️ Architectural Abstractions',
      items: [],
    },
    {
      type: 'category',
      label: '🔒 Enterprise Security',
      items: [],
    },
    {
      type: 'category',
      label: '🧠 Complex Architectures',
      items: [],
    },
    {
      type: 'category',
      label: '🛠️ CLI & Code Factory',
      items: [],
    }
  ],
};

export default sidebars;
