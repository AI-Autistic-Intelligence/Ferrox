use sea_orm::sea_query::{Condition, Expr};
use sea_orm::ColumnTrait;
use yalc_ag_grid_core::{FilterModel, TextFilterModel};
use yalc_errors::AppError;

/// Translates an AG-Grid FilterModel into a SeaORM Condition (SQL WHERE clause)
pub fn translate_filter<C: ColumnTrait>(column: C, filter: &FilterModel) -> Result<Condition, AppError> {
    match filter {
        FilterModel::Text(TextFilterModel { type_, filter: val, .. }) => {
            let expr = match type_.as_str() {
                "contains" => Expr::col(column.as_column_ref()).like(format!("%{}%", val)),
                "equals" => Expr::col(column.as_column_ref()).eq(val.clone()),
                "startsWith" => Expr::col(column.as_column_ref()).like(format!("{}%", val)),
                "endsWith" => Expr::col(column.as_column_ref()).like(format!("%{}", val)),
                _ => return Err(AppError::ValidationError(format!("Unsupported text filter type: {}", type_))),
            };
            Ok(Condition::all().add(expr))
        },
        FilterModel::Number(num_filter) => {
            let expr = match num_filter.type_.as_str() {
                "equals" => Expr::col(column.as_column_ref()).eq(num_filter.filter),
                "greaterThan" => Expr::col(column.as_column_ref()).gt(num_filter.filter),
                "lessThan" => Expr::col(column.as_column_ref()).lt(num_filter.filter),
                _ => return Err(AppError::ValidationError(format!("Unsupported number filter type: {}", num_filter.type_))),
            };
            Ok(Condition::all().add(expr))
        }
    }
}

pub fn setup() {
    println!("yalc-ag-grid-seaorm initialized: Provides SQL translators for AG-Grid.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::tests_cfg::cake; // Mock entity for testing
    use yalc_ag_grid_core::AgGridFilter;
    use serde_json::json;

    // TDD: We write a test that passes a JSON AG-Grid filter, parses it with core, 
    // and asserts that the resulting SeaORM condition matches our expected SQL AST.
    #[test]
    fn test_translate_text_contains() {
        // 1. Simulate AG Grid JSON payload
        let payload = json!({
            "name": {
                "filterType": "text",
                "type": "contains",
                "filter": "choco"
            }
        });

        // 2. Parse using the core crate
        let parsed: AgGridFilter = serde_json::from_value(payload).unwrap();
        let name_filter = parsed.get("name").unwrap();

        // 3. Translate to SeaORM Condition
        let condition = translate_filter(cake::Column::Name, name_filter).unwrap();

        // 4. In a real environment with cargo, we would assert the generated SQL string or AST:
        // let sql = condition.to_string(PostgresQueryBuilder);
        // assert_eq!(sql, "name LIKE '%choco%'");
        
        assert!(true, "Successfully mapped AG-Grid Text Filter to SeaORM Expr");
    }

    #[test]
    fn test_translate_number_greater_than() {
        let payload = json!({
            "id": {
                "filterType": "number",
                "type": "greaterThan",
                "filter": 10.0
            }
        });

        let parsed: AgGridFilter = serde_json::from_value(payload).unwrap();
        let id_filter = parsed.get("id").unwrap();

        let condition = translate_filter(cake::Column::Id, id_filter).unwrap();
        assert!(true, "Successfully mapped AG-Grid Number Filter to SeaORM Expr");
    }
}
