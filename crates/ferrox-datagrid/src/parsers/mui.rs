use crate::models::{DataGridRequest, FilterItem, FilterOperator};
use serde_json::Value;

/// Maps MUI DataGrid's JSON payload to the universal `DataGridRequest`
pub fn parse(payload: Value) -> Result<DataGridRequest, ferrox_errors::AppError> {
    let mut filters = Vec::new();
    
    // MUI usually sends `{ "filterModel": { "items": [ { "field": "name", "operator": "contains", "value": "choco" } ] } }`
    if let Some(items) = payload.get("filterModel").and_then(|v| v.get("items")).and_then(|v| v.as_array()) {
        for item in items {
            if let (Some(field), Some(operator_str), Some(value)) = (
                item.get("field").and_then(|v| v.as_str()),
                item.get("operator").and_then(|v| v.as_str()),
                item.get("value")
            ) {
                let operator = match operator_str {
                    "contains" => FilterOperator::Contains,
                    "equals" => FilterOperator::Equals,
                    "startsWith" => FilterOperator::StartsWith,
                    "endsWith" => FilterOperator::EndsWith,
                    ">" => FilterOperator::GreaterThan,
                    "<" => FilterOperator::LessThan,
                    _ => FilterOperator::Equals,
                };
                
                filters.push(FilterItem {
                    field: field.to_string(),
                    operator,
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
