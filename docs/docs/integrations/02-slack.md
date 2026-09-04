---
sidebar_position: 2
---

# Slack

The `integrations/ferrox-slack` module enables Rust-FERROX to communicate with Slack workspaces.

## Features
- Send alerts and notifications to specific channels
- Format messages using Slack Block Kit
- React to Slack Slash commands securely

## Example

```rust
use ferrox_slack::SlackClient;

let slack = SlackClient::new("xoxb-your-token");
slack.post_message("#alerts", "🚨 High CPU Usage Detected!").await?;
```
