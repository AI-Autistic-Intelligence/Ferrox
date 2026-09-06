# Ferrox Tracing (`ferrox-tracing`)

`ferrox-tracing` integrates OpenTelemetry distributed tracing into Ferrox, exporting trace spans via gRPC/OTLP to Jaeger, Tempo, or Datadog.

## Key Features
- 🛰️ **OpenTelemetry OTLP**: Standardized span collection and export.
- 🔗 **W3C Trace Context**: Propagation of correlation IDs (`traceparent`) across HTTP headers and microservices.
