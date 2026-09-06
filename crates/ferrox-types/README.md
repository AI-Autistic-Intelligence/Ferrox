# Ferrox Types (`ferrox-types`)

`ferrox-types` provides fundamental domain primitives and wrapper types used throughout Ferrox applications,
including validated `Pagination` parameters and type-safe `PublicId` structures.

## Design Rationale
Primitive obsessions (e.g. passing raw `u64` or `String` everywhere) lead to bugs, accidental ID confusion, and invalid query limits.
`ferrox-types` introduces strongly typed abstractions that validate their invariants upon construction.

## Key Features
- 📄 **`Pagination`**: Invariant-enforced page size limit (`limit > 0`) and page offset helper.
- 🏷️ **`PublicId`**: Strongly typed entity identifier wrapper preventing accidental ID substitution.
