use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single AG-Grid Text Filter Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextFilterModel {
    pub filter_type: String, // typically "text"
    pub type_: String,       // e.g., "contains", "equals", "startsWith"
    pub filter: String,      // the actual search term
}

/// Represents an AG-Grid Number Filter Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumberFilterModel {
    pub filter_type: String, // typically "number"
    pub type_: String,       // e.g., "equals", "greaterThan"
    pub filter: f64,
}

/// An enum representing the different possible filter models for a column
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FilterModel {
    Text(TextFilterModel),
    Number(NumberFilterModel),
}

/// The root AG-Grid filter object (a map of column names to their filter models)
pub type AgGridFilter = HashMap<String, FilterModel>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // TDD: We define the JSON structure we expect from AG-Grid frontend, 
    // and assert that our strongly typed Serde definitions can parse it perfectly.
    #[test]
    fn test_deserialize_ag_grid_text_filter() {
        let payload = json!({
            "username": {
                "filterType": "text",
                "type": "contains",
                "filter": "admin"
            }
        });

        let parsed: AgGridFilter = serde_json::from_value(payload).unwrap();
        
        assert!(parsed.contains_key("username"));
        if let FilterModel::Text(text_filter) = &parsed["username"] {
            assert_eq!(text_filter.type_, "contains");
            assert_eq!(text_filter.filter, "admin");
        } else {
            panic!("Expected TextFilterModel");
        }
    }

    #[test]
    fn test_deserialize_ag_grid_number_filter() {
        let payload = json!({
            "age": {
                "filterType": "number",
                "type": "greaterThan",
                "filter": 18.5
            }
        });

        let parsed: AgGridFilter = serde_json::from_value(payload).unwrap();
        
        assert!(parsed.contains_key("age"));
        if let FilterModel::Number(num_filter) = &parsed["age"] {
            assert_eq!(num_filter.type_, "greaterThan");
            assert_eq!(num_filter.filter, 18.5);
        } else {
            panic!("Expected NumberFilterModel");
        }
    }
}
