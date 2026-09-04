use crate::models::{DataGridRequest, FilterOperator};
use bson::{doc, Document, Bson};
use ferrox_errors::AppError;

/// Translates a universal DataGridRequest into a MongoDB Filter Document
pub fn translate_filters(request: &DataGridRequest) -> Result<Document, AppError> {
    let mut query = doc! {};
    
    for filter in &request.filters {
        let bson_val = match filter.value.clone() {
            serde_json::Value::String(s) => Bson::String(s),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Bson::Int64(i)
                } else if let Some(f) = n.as_f64() {
                    Bson::Double(f)
                } else {
                    return Err(AppError::ValidationError("Unsupported number format".to_string()));
                }
            },
            serde_json::Value::Bool(b) => Bson::Boolean(b),
            _ => return Err(AppError::ValidationError("Unsupported filter value type for Mongo".to_string())),
        };
        
        let op_doc = match filter.operator {
            FilterOperator::Equals => doc! { "$eq": bson_val },
            FilterOperator::Contains => {
                if let Bson::String(ref s) = bson_val {
                    doc! { "$regex": format!(".*{}.*", s), "$options": "i" }
                } else {
                    return Err(AppError::ValidationError("Contains operator requires a string".to_string()));
                }
            },
            FilterOperator::GreaterThan => doc! { "$gt": bson_val },
            FilterOperator::LessThan => doc! { "$lt": bson_val },
            _ => return Err(AppError::ValidationError(format!("Unsupported operator for field {}", filter.field))),
        };
        
        query.insert(&filter.field, op_doc);
    }
    
    Ok(query)
}
