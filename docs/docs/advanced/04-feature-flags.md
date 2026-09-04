---
sidebar_position: 4
---

# Feature Flags

The `yalc-feature-flags` crate is used to toggle experimental features across the entire system instantly using Redis Pub/Sub and caching.

## Usage

```rust
use yalc_feature_flags::FeatureFlags;

let is_enabled = FeatureFlags::is_enabled("NEW_DASHBOARD").await;
if is_enabled {
    // Show new dashboard
}
```
