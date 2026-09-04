use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use std::sync::Arc;

/// A simple GraphQL Query root for demonstration.
/// In a real scenario, this would be constructed dynamically or composed of multiple Query roots.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// A simple health check query
    async fn ping(&self) -> &'static str {
        "pong"
    }
}

/// Helper to build a basic Yalc GraphQL schema without mutations or subscriptions
pub fn build_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

/// Helper to build a GraphQL schema with a Repository injected into its context
pub fn build_schema_with_context<T: Send + Sync + 'static>(
    repo: Arc<T>,
) -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(repo)
        .finish()
}

/// Helper to export the GraphQL Schema (SDL) to a file for Frontend Code Generation
pub fn export_sdl<Q, M, S>(schema: &Schema<Q, M, S>, path: &str) -> Result<(), std::io::Error> 
where
    Q: async_graphql::ObjectType + 'static,
    M: async_graphql::ObjectType + 'static,
    S: async_graphql::SubscriptionType + 'static,
{
    let sdl = schema.sdl();
    std::fs::write(path, sdl)?;
    println!("✅ GraphQL Schema successfully exported to {}", path);
    Ok(())
}

pub fn setup() {
    println!("yalc-graphql initialized: Provides async-graphql Schema builders.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::value;
    use serde_json::json;

    // TDD: Verify that we can build a schema and execute a query without any reflection overhead
    #[tokio::test]
    async fn test_graphql_ping_query() {
        let schema = build_schema();

        let request = "{ ping }";
        let response = schema.execute(request).await;

        assert_eq!(
            response.data.into_json().unwrap(),
            json!({
                "ping": "pong"
            })
        );
    }
}
