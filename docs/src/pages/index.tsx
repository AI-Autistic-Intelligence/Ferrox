import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import styles from './index.module.css';

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--dark', styles.heroBanner)}>
      <div className="container">
        <div className={styles.logoContainer}>
          <img src="/img/logo.jpg" alt="Ferrox Logo" className={styles.heroLogo} />
        </div>
        <Heading as="h1" className="hero__title">
          ⚡ Ferrox Framework
        </Heading>
        <p className="hero__subtitle">
          A Progressive, Enterprise-Grade Server-Side Framework for Rust
        </p>
        <p className={styles.heroTagline}>
          Bringing the Developer Experience of NestJS & Angular to the unmatched performance of Tokio & Axum.
        </p>

        <div className={styles.buttons}>
          <Link
            className="button button--primary button--lg"
            to="/docs/overview/introduction">
            🚀 Getting Started
          </Link>
          <Link
            className="button button--danger button--lg"
            href="https://www.paypal.com/donate/?hosted_button_id=9Q3UG829FHT6J">
            ❤️ Sponsor Project
          </Link>
          <Link
            className="button button--secondary button--lg"
            href="https://discord.gg/Bx3CzGec7d">
            💬 Discord
          </Link>
          <Link
            className="button button--outline button--lg"
            href="https://www.reddit.com/r/Ferrox/">
            📱 Reddit
          </Link>
        </div>

        <div className={styles.installSnippet}>
          <code>cargo install cargo-ferrox</code>
        </div>
      </div>
    </header>
  );
}

const FeatureList = [
  {
    title: '🛡️ Zero-Trust Security',
    description: (
      <>
        Built-in PASETO & JWT token engines, dual-token refresh, declarative 
        role guards (<code>RequireRole</code>), and HMAC webhook verification.
      </>
    ),
  },
  {
    title: '⚡ Cache Stampede Protection',
    description: (
      <>
        Integrated <code>Singleflight</code> pattern powered by Tokio broadcast 
        channels to prevent dogpile effects on high-concurrency endpoints.
      </>
    ),
  },
  {
    title: '🔄 Resilience & Fault Tolerance',
    description: (
      <>
        Redis-backed rate limiters, circuit breakers (Closed/Open/HalfOpen 
        state machine), and distributed synchronization out of the box.
      </>
    ),
  },
  {
    title: '🧠 Enterprise Architectures',
    description: (
      <>
        Decoupled <code>CommandBus</code> and <code>QueryBus</code> CQRS dispatchers, 
        Saga orchestrators for distributed transactions, and background jobs.
      </>
    ),
  },
  {
    title: '🛠️ Code Factory & AutoZod',
    description: (
      <>
        Automated generic CRUD routing macros (<code>crud_router!</code>) and 
        strongly-typed <code>ValidatedJson&lt;T&gt;</code> payload validation.
      </>
    ),
  },
  {
    title: '🌐 Multi-Transport Engine',
    description: (
      <>
        Unified application lifecycle manager supporting HTTP (Axum), gRPC, 
        WebSockets, Server-Sent Events (SSE), and GraphQL schema generators.
      </>
    ),
  },
];

function Feature({ title, description }: { title: string; description: JSX.Element }) {
  return (
    <div className={clsx('col col--4', styles.featureCardCol)}>
      <div className={clsx('card', styles.featureCard)}>
        <div className="card__header">
          <Heading as="h3">{title}</Heading>
        </div>
        <div className="card__body">
          <p>{description}</p>
        </div>
      </div>
    </div>
  );
}

export default function Home(): JSX.Element {
  return (
    <Layout
      title="Ferrox - Enterprise Server-Side Framework for Rust"
      description="A Progressive, Enterprise-Grade Server-Side Framework for Rust powered by Tokio & Axum. DX of NestJS, speed of Rust.">
      <HomepageHeader />
      <main className="container margin-vert--xl">
        <section className="margin-bottom--xl">
          <div className="text--center margin-bottom--lg">
            <Heading as="h2">Why Choose Ferrox?</Heading>
            <p>Built from the ground up for massive scalability, zero-trust security, and instant developer productivity.</p>
          </div>
          <div className="row">
            {FeatureList.map((props, idx) => (
              <Feature key={idx} {...props} />
            ))}
          </div>
        </section>

        <section className={clsx('margin-vert--xl', styles.codeSection)}>
          <div className="row row--align-center">
            <div className="col col--6">
              <Heading as="h2">Quick Start in 20 Seconds</Heading>
              <p>
                Ferrox combines modularity, speed, and safety. Initialize your multi-transport server with minimal boilerplate.
              </p>
              <p>
                Need support or have questions? Email us directly at{' '}
                <a href="mailto:info@ferrox-rust.dev">info@ferrox-rust.dev</a>.
              </p>
            </div>
            <div className="col col--6">
              <pre className={styles.codeBlock}>
                <code>{`use axum::{routing::get, Json, Router};
use ferrox_app::FerroxApp;
use ferrox_transports::http::HttpTransport;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new().route(
        "/api/v1/ping",
        get(|| async { Json(json!({ "status": "ok", "framework": "Ferrox" })) }),
    );

    let transport = HttpTransport::new(router, 3000);

    FerroxApp::new()
        .add_transport(transport)
        .start()
        .await?;

    Ok(())
}`}</code>
              </pre>
            </div>
          </div>
        </section>

        <section className="margin-vert--xl text--center">
          <div className="card padding--xl" style={{ backgroundColor: 'rgba(255, 255, 255, 0.03)', borderRadius: '16px', border: '1px solid rgba(255, 255, 255, 0.1)' }}>
            <Heading as="h2">💖 Support Ferrox Open-Source Development</Heading>
            <p style={{ maxWidth: '750px', margin: '0 auto 1.5rem auto', fontSize: '1.1rem' }}>
              Developing a 45-crate ecosystem, running benchmark suites, hosting domain services (<code>ferrox-rust.dev</code>), and maintaining multi-language docs requires ongoing servers and infrastructure. Your donations directly power rapid releases and continuous open-source development!
            </p>
            <div className="row row--align-center" style={{ justifyContent: 'center' }}>
              <div className="col col--6 text--center">
                <a href="https://www.paypal.com/donate/?hosted_button_id=9Q3UG829FHT6J" target="_blank" rel="noopener noreferrer">
                  <img src="/img/paypal_qr.png" alt="Scan to donate via PayPal" style={{ width: '180px', borderRadius: '12px', boxShadow: '0 4px 16px rgba(0,0,0,0.3)', marginBottom: '1rem' }} />
                </a>
                <div>
                  <Link
                    className="button button--danger button--lg"
                    href="https://www.paypal.com/donate/?hosted_button_id=9Q3UG829FHT6J">
                    💳 Donate via PayPal
                  </Link>
                  <span style={{ margin: '0 0.5rem' }}></span>
                  <Link
                    className="button button--outline button--secondary button--lg"
                    to="/docs/community/donations">
                    📖 Read Donation Info
                  </Link>
                </div>
              </div>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
