---
sidebar_position: 6
---

# ⏰ Cron Task Scheduling

Many applications require periodic background tasks—such as clearing expired sessions every night at midnight, recalculating analytics hourly, or polling external feeds every 5 minutes.

`ferrox-schedule` provides cron-based task scheduling for background worker processes.

---

## 1. Defining Cron Tasks

`ferrox-schedule` allows registering async closures scheduled with standard 5-field cron syntax:

```rust
use std::time::Duration;
use tokio::time::interval;

pub async fn schedule_daily_cleanup() {
    tokio::spawn(async {
        // Runs periodic cleanup job
        let mut timer = interval(Duration::from_secs(86400)); // Every 24 hours
        loop {
            timer.tick().await;
            println!("🧹 Running scheduled daily database cleanup...");
            // Execute cleanup logic
        }
    });
}
```

---

## 2. Standard Cron Expression Syntax

| Expression | Execution Schedule |
|---|---|
| `*/5 * * * *` | Every 5 minutes |
| `0 * * * *` | Every hour on the hour |
| `0 0 * * *` | Daily at midnight |
| `0 0 * * 0` | Every Sunday at midnight |

---

## 3. Distributed Task Locking

> [!WARNING]
> When running multiple container replicas of your Ferrox app in Kubernetes, simple local cron timers will fire on **every replica simultaneously**. Combine `ferrox-schedule` with `ferrox-sync` distributed locks so only **one replica** executes the periodic task per interval.
