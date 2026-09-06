# Ferrox Utils (`ferrox-utils`)

`ferrox-utils` contains shared utility functions, date/time UTC formatters, string casing converters, and UUID helpers
used across the Ferrox framework ecosystem.

## Principles
All systems within Ferrox strictly enforce UTC timestamps and standardized string formatting across database boundaries.
`ferrox-utils` centralizes these core helper functions to prevent code duplication across service layers.

## Key Features
- 📅 **Date & Time Helpers**: Guarantees UTC timestamps (`now_utc()`) and HTTP GMT date formatting.
- 🔤 **String Case Conversion**: Convert strings between `snake_case`, `camelCase`, `PascalCase`, and `kebab-case`.
- 🔑 **UUID Utilities**: Generates standard v4 UUIDs and short URL-safe identifiers.
