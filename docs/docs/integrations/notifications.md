---
sidebar_position: 3
---

# 🔔 Notifications & Slack Integration

Real-time developer notifications and DevOps alert dispatching (for server outages, critical errors, or high-value purchase events) require webhook integration with team chat tools.

`integrations/ferrox-notifications-slack` provides a client for dispatching Slack incoming webhooks.

---

## 1. Dispatching Slack Notifications

```rust
use ferrox_notifications_slack::SlackClient;
use ferrox_errors::AppError;

let slack = SlackClient::new("https://hooks.slack.com/services/T00/B00/X00");

// Send markdown-formatted alert message to Slack channel
slack.send_alert(
    "🚨 CRITICAL ALERT: Database Connection Pool Exhausted!",
    "#alerts-critical"
).await?;
```

---

## 2. Structured Block Kit Messages

```rust
use serde_json::json;

let block_payload = json!({
    "blocks": [
        {
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "*New Enterprise Signup!* 🚀\n*Company:* Acme Corp"
            }
        }
    ]
});

slack.send_raw_payload(block_payload).await?;
```
