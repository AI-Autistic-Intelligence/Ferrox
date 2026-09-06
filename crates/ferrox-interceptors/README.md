# Ferrox Interceptors (`ferrox-interceptors`)

`ferrox-interceptors` provides request/response lifecycle hooks and middleware interceptors (e.g. `CacheInterceptor`, `TimingInterceptor`)
that inspect, transform, or cache HTTP responses.

## Key Features
- 🔄 **`Interceptor` Trait**: Modify requests before route execution and transform responses afterwards.
- ⚡ **`CacheInterceptor`**: Automated response caching based on HTTP headers and cache keys.
