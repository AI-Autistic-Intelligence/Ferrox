# Ferrox Database Redis (`ferrox-database-redis`)

`ferrox-database-redis` provides Redis integration for caching, session storage, distributed rate limiting, and real-time Pub/Sub.

## Key Features
- ⚡ **Multiplexed Connection Pool**: Efficient async Redis client backed by `bb8` or `redis-rs`.
- 🔑 **Cache Helper Operations**: Strongly typed `get_json`, `set_json`, `expire`, and `del` primitives.
- 📻 **Pub/Sub Subscriptions**: Asynchronous message receiver streams.
