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
        use yalc_database_core::Repository;

        // Ensure the State extraction uses Arc<$repo>
        Router::<Arc<$repo>>::new()
            .route(
                "/",
                get(|State(repo): State<Arc<$repo>>| async move {
                    let items = repo.find_all().await?;
                    Ok::<_, yalc_errors::AppError>(Json(items))
                }),
            )
            .route(
                "/:id",
                get(|Path(id): Path<$id>, State(repo): State<Arc<$repo>>| async move {
                    match repo.find_by_id(id).await? {
                        Some(item) => Ok::<_, yalc_errors::AppError>(Json(item)),
                        None => Err(yalc_errors::AppError::NotFound("Entity not found".into())),
                    }
                }),
            )
            .route(
                "/",
                post(|State(repo): State<Arc<$repo>>, Json(payload): Json<$entity>| async move {
                    let created = repo.insert(payload).await?;
                    Ok::<_, yalc_errors::AppError>(Json(created))
                }),
            )
            .route(
                "/:id",
                patch(|Path(id): Path<$id>, State(repo): State<Arc<$repo>>, Json(payload): Json<$entity>| async move {
                    let updated = repo.update(id, payload).await?;
                    Ok::<_, yalc_errors::AppError>(Json(updated))
                }),
            )
            .route(
                "/:id",
                delete(|Path(id): Path<$id>, State(repo): State<Arc<$repo>>| async move {
                    repo.delete(id).await?;
                    Ok::<_, yalc_errors::AppError>(axum::http::StatusCode::NO_CONTENT)
                }),
            )
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use axum::Router;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use yalc_database_core::Repository;
    use yalc_errors::AppError;

    // 1. Define a dummy entity
    #[derive(Clone, Serialize, Deserialize)]
    struct User {
        id: uuid::Uuid,
        name: String,
    }

    // 2. Define a dummy repository
    struct MockUserRepository;

    #[async_trait]
    impl Repository<User, uuid::Uuid> for MockUserRepository {
        async fn find_by_id(&self, _id: uuid::Uuid) -> Result<Option<User>, AppError> { Ok(None) }
        async fn find_all(&self) -> Result<Vec<User>, AppError> { Ok(vec![]) }
        async fn insert(&self, entity: User) -> Result<User, AppError> { Ok(entity) }
        async fn update(&self, _id: uuid::Uuid, entity: User) -> Result<User, AppError> { Ok(entity) }
        async fn delete(&self, _id: uuid::Uuid) -> Result<(), AppError> { Ok(()) }
    }

    // 3. Test that the macro compiles and returns a valid Axum Router bound to the Repo state
    #[test]
    fn test_crud_router_generation() {
        let _router: Router<Arc<MockUserRepository>> = crud_router!(User, uuid::Uuid, MockUserRepository);
        assert!(true, "Router generated successfully!");
    }
}
