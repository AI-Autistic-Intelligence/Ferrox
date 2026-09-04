use crate::models::{DataGridRequest, FilterItem, FilterOperator};
use serde_json::Value;

/// Maps TanStack Table's JSON payload to the universal `DataGridRequest`
pub fn parse(payload: Value) -> Result<DataGridRequest, ferrox_errors::AppError> {
    let mut filters = Vec::new();
    
    // Tanstack usually sends `{ "id": "name", "value": "choco" }` inside a columnFilters array.
    if let Some(items) = payload.get("columnFilters").and_then(|v| v.as_array()) {
        for item in items {
            if let (Some(field), Some(value)) = (
                item.get("id").and_then(|v| v.as_str()),
                item.get("value")
            ) {
                filters.push(FilterItem {
                    field: field.to_string(),
                    operator: FilterOperator::Contains, // Default operator for basic tanstack filter
                    value: value.clone(),
                });
            }
        }
    }
    
    Ok(DataGridRequest {
        pagination: None,
        sorts: vec![],
        filters,
    })
}
