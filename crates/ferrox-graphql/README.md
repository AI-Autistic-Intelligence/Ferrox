# Ferrox GraphQL (`ferrox-graphql`)

`ferrox-graphql` integrates `async-graphql` into the Ferrox framework, allowing developers to quickly build, serve,
and inspect GraphQL schemas alongside HTTP REST endpoints.

## Architectural Context
Modern backend architectures often serve mobile applications or frontend dashboards that demand precise data fetching.
`ferrox-graphql` provides high-performance GraphQL schema execution with built-in support for GraphQL Playground
and automatic Schema Definition Language (SDL) export.

## Key Features
- ⚡ **`async-graphql` Integration**: Full support for Queries, Mutations, and Subscriptions.
- 🎮 **Interactive Playground**: Built-in IDE route for testing queries in development.
- 📜 **SDL Export**: Programmatically generate `.graphql` schema files for client codegen.
