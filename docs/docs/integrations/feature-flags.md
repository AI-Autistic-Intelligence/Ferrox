---
sidebar_position: 4
---

# 🚩 Feature Flags & Toggles

Feature flags allow developers to safely roll out new features to a subset of users, perform A/B testing, or instantaneously disable failing endpoints without redeploying code.

`integrations/ferrox-feature-flags` provides feature flag evaluation tools.

---

## 1. Evaluating Feature Flags

```rust
use ferrox_feature_flags::FeatureFlagEngine;

let flags = FeatureFlagEngine::new();

// Evaluate flag for a specific user ID
if flags.is_enabled("new_dashboard_v2", "user_101") {
    println!("Rendering Dashboard V2");
} else {
    println!("Rendering Legacy Dashboard V1");
}
```

---

## 2. Dynamic Remote Toggle Provider (Unleash / Local)

`FeatureFlagEngine` supports loading flag states dynamically from environment variables, Redis, or Unleash servers.
