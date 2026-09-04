/// A macro that generates standard CRUD routes for a given entity and repository type.
/// The router expects the repository to be provided via Axum's State extraction.
#[macro_export]
macro_rules! crud_router {
    ($entity:ty, $id:ty, $repo:ty) => {{
        use axum::{
            extract::{Path, State},
            routing::{get, post, delete, patch},
            Json, Router,
        };
        use std::sync::Arc;
        use ferrox_database_core::Repository;
        use ferrox_validation::ValidatedJson;

        // Ensure the State extraction uses Arc<$repo>
        Router::<Arc<$repo>>::new()
            .route(
                "/",
                get(|State(repo): State<Arc<$repo>>| async move {
                    let items = repo.find_all().await?;
                    Ok::<_, ferrox_errors::AppError>(Json(items))
                }),
            )
            .route(
                "/:id",
                get(|Path(id): Path<$id>, State(repo): State<Arc<$repo>>| async move {
                    match repo.find_by_id(id).await? {
                        Some(item) => Ok::<_, ferrox_errors::AppError>(Json(item)),
                        None => Err(ferrox_errors::AppError::NotFound("Entity not found".into())),
                    }
                }),
            )
            .route(
                "/",
                // Notice the use of ValidatedJson here for AutoZod validation!
                post(|State(repo): State<Arc<$repo>>, ValidatedJson(payload): ValidatedJson<$entity>| async move {
                    let created = repo.insert(payload).await?;
                    Ok::<_, ferrox_errors::AppError>(Json(created))
                }),
            )
            .route(
                "/:id",
                patch(|Path(id): Path<$id>, State(repo): State<Arc<$repo>>, ValidatedJson(payload): ValidatedJson<$entity>| async move {
                    let updated = repo.update(id, payload).await?;
                    Ok::<_, ferrox_errors::AppError>(Json(updated))
                }),
            )
            .route(
                "/:id",
                delete(|Path(id): Path<$id>, State(repo): State<Arc<$repo>>| async move {
                    repo.delete(id).await?;
                    Ok::<_, ferrox_errors::AppError>(axum::http::StatusCode::NO_CONTENT)
                }),
            )
    }};
}

/// A Code Factory macro that generates a full vertical slice for an Entity.
/// It creates:
/// - A Create DTO with AutoZod validation.
/// - A GraphQL Object derivation.
/// - A REST Router using `crud_router!`.
#[macro_export]
macro_rules! vertical_slice {
    ($name:ident, $id:ty, $repo:ty, { $($field:ident: $ftype:ty),* }) => {
        // 1. DTO Generation with AutoZod Validation
        // Note: The caller must have `serde` and `validator` in scope.
        #[derive(Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
        // If async-graphql is enabled, it automatically becomes a GraphQL Input/Object too!
        #[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject, async_graphql::InputObject))]
        pub struct $name {
            $(
                pub $field: $ftype,
            )*
        }

        // 2. Generate REST Router (which uses the ValidatedJson automatically)
        pub fn router() -> axum::Router<std::sync::Arc<$repo>> {
            $crate::crud_router!($name, $id, $repo)
        }
    };
}
