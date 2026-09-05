import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

/**
 * Ferrox Docusaurus Sidebars Configuration (Level 2 Enterprise Edition)
 */
const sidebars: SidebarsConfig = {
  ferroxSidebar: [
    {
      type: 'category',
      label: '🚀 Overview',
      items: [
        'overview/introduction',
        'overview/first-steps',
        'overview/lifecycle',
      ],
    },
    {
      type: 'category',
      label: '🎓 Tutorial: Mini-Project',
      items: [
        'tutorial/setup',
        'tutorial/building-the-core',
        'tutorial/code-factory',
      ],
    },
    {
      type: 'category',
      label: '🏗️ Fundamentals',
      items: [
        'fundamentals/controllers',
        'fundamentals/providers',
        'fundamentals/middlewares',
        'fundamentals/interceptors',
        'fundamentals/errors',
        'fundamentals/configuration',
        'fundamentals/testing',
      ],
    },
    {
      type: 'category',
      label: '🛡️ Architectural Abstractions',
      items: [
        'abstractions/pipes',
        'abstractions/guards',
        'abstractions/validation',
        'abstractions/crud-generator',
      ],
    },
    {
      type: 'category',
      label: '💾 Databases & Persistence',
      items: [
        'databases/overview',
        'databases/seaorm',
        'databases/mongodb',
        'databases/redis',
        'databases/migrations',
      ],
    },
    {
      type: 'category',
      label: '🔒 Security & Resilience',
      items: [
        'security/jwt',
        'security/advanced-auth',
        'security/rate-limiting',
        'security/circuit-breaker',
        'security/singleflight',
        'security/distributed-locks',
      ],
    },
    {
      type: 'category',
      label: '🧠 Complex Architectures',
      items: [
        'architectures/api-gateway',
        'architectures/cqrs',
        'architectures/caching',
        'architectures/sagas',
        'architectures/events',
        'architectures/queues-jobs',
        'architectures/task-scheduling',
      ],
    },
    {
      type: 'category',
      label: '📊 Observability & Monitoring',
      items: [
        'observability/logging',
        'observability/health-checks',
        'observability/metrics',
        'observability/tracing',
      ],
    },
    {
      type: 'category',
      label: '🌐 Transports & Realtime Data',
      items: [
        'transports/transports-overview',
        'transports/graphql',
        'transports/graphql-advanced',
        'transports/sse',
        'transports/file-storage',
        'transports/datagrid',
      ],
    },
    {
      type: 'category',
      label: '🔌 Ecosystem & Integrations',
      items: [
        'integrations/mailer',
        'integrations/payments',
        'integrations/notifications',
        'integrations/feature-flags',
        'integrations/webhooks',
        'integrations/reports-and-cloud',
        'integrations/i18n',
      ],
    },
    {
      type: 'category',
      label: '🏭 Production Deployment & DevOps',
      items: [
        'deployment/docker-kubernetes',
        'deployment/ci-cd',
      ],
    },
    {
      type: 'category',
      label: '⚡ Performance & Optimization',
      items: [
        'performance/benchmarks',
        'performance/profiling',
      ],
    },
    {
      type: 'category',
      label: '🛠️ CLI & Tooling',
      items: [
        'cli/code-factory',
        'cli/commands-reference',
      ],
    },
    {
      type: 'category',
      label: '💖 Community & Support',
      items: [
        'community/donations',
      ],
    },
  ],
};

export default sidebars;
