use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::persistence::UserRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub onboarding_completed: Option<bool>,
}

pub fn router(user_repo: Arc<dyn UserRepository>) -> Router {
    Router::new()
        .route("/profile/:id", get(get_profile_handler))
        .route("/profile/:id", put(update_profile_handler))
        .route("/onboarding", post(complete_onboarding_handler))
        .with_state(user_repo)
}

async fn get_profile_handler(
    State(user_repo): State<Arc<dyn UserRepository>>,
    Path(id): Path<String>,
) -> Json<Value> {
    match user_repo.find_by_id(&id).await {
        Ok(Some(user)) => Json(json!({ "success": true, "user": user })),
        _ => Json(json!({ "success": false, "error": "User not found" })),
    }
}

async fn update_profile_handler(
    State(user_repo): State<Arc<dyn UserRepository>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Json<Value> {
    if let Ok(Some(mut user)) = user_repo.find_by_id(&id).await {
        if let Some(name) = payload.full_name { user.full_name = name; }
        if let Some(avatar) = payload.avatar_url { user.avatar_url = Some(avatar); }
        if let Some(onboarding) = payload.onboarding_completed { user.onboarding_completed = onboarding; }

        let _ = user_repo.save(&user).await;
        return Json(json!({ "success": true, "user": user }));
    }
    Json(json!({ "success": false, "error": "User not found" }))
}

async fn complete_onboarding_handler(
    State(user_repo): State<Arc<dyn UserRepository>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let user_id = payload["user_id"].as_str().unwrap_or("usr_admin_01");
    if let Ok(Some(mut user)) = user_repo.find_by_id(user_id).await {
        user.onboarding_completed = true;
        let _ = user_repo.save(&user).await;
        return Json(json!({ "success": true, "message": "Onboarding completed successfully" }));
    }
    Json(json!({ "success": false, "error": "Failed to update onboarding state" }))
}
