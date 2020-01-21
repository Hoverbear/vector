import React from 'react';

import CodeBlock from '@theme/CodeBlock';
import TabItem from '@theme/TabItem';
import Tabs from '@theme/Tabs';

function InstallationCommand() {
  return (
    <Tabs
      className="mini"
      defaultValue="humans"
      values={[
        { label: <><i className="feather icon-user-check"></i> For Humans</>, value: 'humans', },
        { label: <><i className="feather icon-cpu"></i> For Machines</>, value: 'machines', },
      ]
    }>
      <TabItem value="humans">
        <CodeBlock className="language-bash">
          curl --proto '=https' --tlsv1.2 -sSf https://sh.vector.dev | sh
        </CodeBlock>
      </TabItem>
      <TabItem value="machines">
        <CodeBlock className="language-bash">
          curl --proto '=https' --tlsv1.2 -sSf https://sh.vector.dev | sh -s -- -y
        </CodeBlock>
      </TabItem>
    </Tabs>
  );
}

export default InstallationCommand;
