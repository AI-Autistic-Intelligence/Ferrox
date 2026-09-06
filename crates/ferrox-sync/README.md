# Ferrox Sync (`ferrox-sync`)

`ferrox-sync` provides distributed locks and sync mechanisms across multi-node Ferrox deployments, preventing race conditions
during critical scheduled tasks or shared resource modifications.

## Key Features
- 🔒 **Distributed Locks**: Backed by Redis Redlock algorithm or SQL advisory locks.
- ⏱️ **Auto-Expiring Lease**: Prevents deadlocks by enforcing automatic lock lease timeouts.
