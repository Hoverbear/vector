import React, { useState, useEffect } from 'react';

import Alert from '@site/src/components/Alert';
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
    <Layout title="Collect Docker Logs, Metrics, & Events In Minutes" description="Collect Docker logs, metrics, and events in minutes, for free. Quickly collect Docker logs and metrics and send them to one or more destinations.">
      <header className="hero domain-bg domain-bg--platforms">
        <div className="container container--narrow">
          <div className="component-icons">
            <div className="icon panel">
              <img src="/img/logos/docker.png" alt="Docker" />
            </div>
            <a href="#" className="icon panel" title="Select a destination">
              <i className="feather icon-plus"></i>
            </a>
          </div>
          <h1>Collect Docker Logs &amp; Events In Minutes</h1>
          <p>written and maintained, with love, by the <Link to="/community/#team">Vector team</Link></p>
        </div>
      </header>
      <main className="container container--narrow long-form margin-vert--xl">
        <section>
          <blockquote className="blockquote--primary">
            <div>&quot;I just wanna, like, collect my Docker logs and grep them -- why is all of this so complicated?&quot;</div>
            <footer>— all developers</footer>
          </blockquote>

          <p>Did you know that Docker splits long log messages? Or that collecting Docker logs, metrics, and events requires three separate APIs? Or that enriching this data with Docker context requires yet another API? Don't sweat it! We've got you covered. Below is a simple guide that uses a best-in-class tool (Vector) for collecting Docker logs and handling all of these edge cases. You'll be setup in minutes so you can get back to doing what you do best.</p>
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
              <AnchoredH3 id="configure">Add one or more "sinks" (destinations)</AnchoredH3>

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
