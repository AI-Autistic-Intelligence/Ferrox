# Ferrox Singleflight (`ferrox-singleflight`)

`ferrox-singleflight` provides cache stampede (dogpile effect) suppression. When multiple concurrent requests attempt to compute
or fetch the same missing key simultaneously, `Singleflight` ensures only **one** execution occurs while sharing the result across all callers.

## Rationale
High-concurrency systems often suffer from cache stampedes when a popular cache key expires: thousands of incoming requests miss the cache
simultaneously and hammer the database. `Singleflight` intercepts duplicate key lookups in-flight using Tokio broadcast channels.

## Key Features
- ⚡ **Duplicate Suppression**: Only 1 worker executes the expensive task; all other waiters receive the cloned result.
- 🛡️ **Memory Efficient**: In-flight computations are freed immediately upon completion.
