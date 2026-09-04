use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct DataGridRequest {
    pub pagination: Option<Pagination>,
    pub sorts: Vec<SortItem>,
    pub filters: Vec<FilterItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct SortItem {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum, Copy, Eq))]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct FilterItem {
    pub field: String,
    pub operator: FilterOperator,
    /// We use an inline scalar type or just a String for GraphQL flexibility, 
    /// but serde_json::Value maps better to generic JSON for REST.
    /// In async-graphql, Value maps directly to a JSON scalar!
    #[cfg_attr(feature = "graphql", graphql(type = "async_graphql::types::Json<serde_json::Value>"))]
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum, Copy, Eq))]
pub enum FilterOperator {
    Contains,
    Equals,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
}
