use crate::models::{DataGridRequest, FilterItem, FilterOperator};
use serde_json::Value;
use std::collections::HashMap;

/// Maps AG-Grid's complex JSON payload to the universal `DataGridRequest`
pub fn parse(payload: Value) -> Result<DataGridRequest, yalc_errors::AppError> {
    // Basic mock implementation of AG-Grid parser
    // In a real scenario, this recursively parses FilterModels and SortModels from AG-Grid.
    let mut filters = Vec::new();
    
    if let Some(filter_model) = payload.get("filterModel").and_then(|v| v.as_object()) {
        for (field, filter_val) in filter_model {
            if let Some(type_str) = filter_val.get("type").and_then(|v| v.as_str()) {
                let operator = match type_str {
                    "contains" => FilterOperator::Contains,
                    "equals" => FilterOperator::Equals,
                    "startsWith" => FilterOperator::StartsWith,
                    "endsWith" => FilterOperator::EndsWith,
                    "greaterThan" => FilterOperator::GreaterThan,
                    "lessThan" => FilterOperator::LessThan,
                    _ => FilterOperator::Equals,
                };
                
                let value = filter_val.get("filter").cloned().unwrap_or(Value::Null);
                
                filters.push(FilterItem {
                    field: field.clone(),
                    operator,
                    value,
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
