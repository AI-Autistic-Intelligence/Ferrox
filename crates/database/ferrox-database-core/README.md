# Ferrox Database Core (`ferrox-database-core`)

`ferrox-database-core` defines the abstract persistence layer for Ferrox applications, introducing the generic `Repository<T, Id>` trait.

## Inversion of Control & Persistence Decoupling
By depending on `Repository<T, Id>` traits rather than concrete database drivers (SQL, Mongo, Redis), domain logic and services stay
completely decoupled from database engine details. This enables seamless unit testing via in-memory mock repositories.

## Key Features
- 📥 **Generic `Repository<Entity, Id>` Trait**: Standard `find_by_id`, `find_all`, `save`, `update`, and `delete` methods.
- 🧪 **Testing Mocks**: Simplify unit tests without launching database containers.
