import React, { useState, useEffect } from 'react';

import Heading from '@theme/Heading';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

import classnames from 'classnames';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';

const AnchoredH2 = Heading('h2');
const AnchoredH3 = Heading('h3');

function Community() {
  const context = useDocusaurusContext();
  const {siteConfig = {}} = context;
  const {metadata: {team}} = siteConfig.customFields;

  return (
    <Layout title="Collect Docker Logs, Metrics, & Events In Minutes" description="Collect Docker logs, metrics, and events in minutes, for free.">
      <header className="hero domain-bg domain-bg--platforms">
        <div className="container container--narrow">
          <div className="hero--component-icons">
            <div class="icon panel">
              <img src="/img/logos/docker.png" alt="Docker" />
            </div>
          </div>
          <h1>Collect Docker Logs, Metrics, &amp; Events In Minutes, For Free</h1>
          <p>Written and maintained by the <Link to="/community/#team">Vector team</Link></p>
        </div>
      </header>
      <main className="container container--narrow margin-vert--xl">
        <section>
          <p>Collecting Docker logs and metrics is an essential task if you want to observe and monitor your Docker services, but doing so is easier said than done. How do you collect both logs and metrics? How do you enrich logs and metrics with the proper Docker context? And how do you so without spending hours reading manuals? We'll show you how in this guide.</p>
        </section>
        <section>
          <AnchoredH2 id="about">About this guide</AnchoredH2>

          <ol>
            <li>We'll use Vector to collect Docker logs _and_ metrics. Why Vector? See below.</li>
            <li>We'll enrich all data collected with useful Docker context.</li>
            <li>We'll show you how to send (fan-out) this data to any destination(s) you choose.</li>
            <li>And we'll do it all in a few short minutes.</li>
          </ol>
        </section>
        <section>
          <AnchoredH2 id="guide">A simple step-by-step guide</AnchoredH2>

          <ol className="guide">
            <li>
              <AnchoredH3 id="install-vector">Install Vector</AnchoredH3>

              <p>
                Vector is a modern log and metrics collector written in Rust. It's reliable, fast, and simpl to setup. It comes pre-loaded with Docker integration making it easy to setup.
              </p>

              [install]
            </li>

            <li>
              <AnchoredH3 id="configure">Configure Vector with the Docker source</AnchoredH3>

              <p>fsdfdsf</p>
            </li>

            <li>
              <AnchoredH3 id="configure">Run Vector &amp; verify collection</AnchoredH3>

              <p>fsdfdsf</p>
            </li>

            <li>
              <AnchoredH3 id="configure">Choose one or more "sinks" (destinations)</AnchoredH3>

              <p>fsdfdsf</p>
            </li>
          </ol>

        </section>
        <section>
          <h2>Next steps</h2>

        </section>
        <section>
          <h2>Why Vector?</h2>

        </section>
        <section>
          <h2>Learn more &amp; get help</h2>

        </section>
      </main>
    </Layout>
  );
}

export default Community;
