use crate::models::{DataGridRequest, FilterOperator};
use sea_orm::sea_query::{Condition, Expr, SimpleExpr};
use sea_orm::{ColumnTrait, EntityTrait};
use yalc_errors::AppError;
use std::str::FromStr;

/// Translates a universal DataGridRequest into a SeaORM Condition (SQL WHERE clause)
pub fn translate_filters<E>(request: &DataGridRequest) -> Result<Condition, AppError> 
where
    E: EntityTrait,
    <<E as EntityTrait>::Column as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut condition = Condition::all();
    
    for filter in &request.filters {
        // Find the column by name dynamically
        let col = E::Column::from_str(&filter.field)
            .map_err(|e| AppError::ValidationError(format!("Invalid column {}: {}", filter.field, e)))?;
        
        let expr: SimpleExpr = match filter.operator {
            FilterOperator::Contains => Expr::col(col.as_column_ref()).like(format!("%{}%", filter.value.as_str().unwrap_or(""))),
            FilterOperator::Equals => {
                if let Some(s) = filter.value.as_str() {
                    Expr::col(col.as_column_ref()).eq(s)
                } else if let Some(n) = filter.value.as_f64() {
                    Expr::col(col.as_column_ref()).eq(n)
                } else {
                    Expr::col(col.as_column_ref()).eq(filter.value.to_string())
                }
            },
            FilterOperator::GreaterThan => Expr::col(col.as_column_ref()).gt(filter.value.as_f64().unwrap_or(0.0)),
            FilterOperator::LessThan => Expr::col(col.as_column_ref()).lt(filter.value.as_f64().unwrap_or(0.0)),
            _ => return Err(AppError::ValidationError(format!("Unsupported operator for field {}", filter.field))),
        };
        
        condition = condition.add(expr);
    }
    
    Ok(condition)
}
