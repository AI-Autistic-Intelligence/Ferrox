# Ferrox Database Mongo (`ferrox-database-mongo`)

`ferrox-database-mongo` provides document storage capabilities for Ferrox applications, wrapping the official MongoDB Rust driver
with strongly-typed `MongoRepository` traits and BSON conversion utilities.

## Key Features
- 🍃 **MongoDB Async Client**: Managed database connection pooling.
- 📦 **Document Repository**: Generic CRUD methods for BSON-serializable domain models.
- 🔍 **Aggregation Builders**: Helpers for building MongoDB pipeline queries.
