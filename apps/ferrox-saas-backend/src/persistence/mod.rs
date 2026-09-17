use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: String, // "admin", "user"
    pub totp_enabled: bool,
    pub totp_secret: Option<String>,
    pub onboarding_completed: bool,
    pub avatar_url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedUserSegment {
    pub id: String,
    pub name: String,
    pub filter_query: String, // e.g. "role:user AND onboarding:true"
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLogEntry {
    pub correlation_id: String,
    pub method: String,
    pub path: String,
    pub status_code: u16,
    pub duration_ms: u128,
    pub timestamp: String,
    pub user_agent: String,
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<UserEntity>, anyhow::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, anyhow::Error>;
    async fn save(&self, user: &UserEntity) -> Result<(), anyhow::Error>;
    async fn list_users(&self, limit: usize, offset: usize) -> Result<Vec<UserEntity>, anyhow::Error>;
    async fn count_users(&self) -> Result<u64, anyhow::Error>;
}

// In-Memory Fallback & SQL/NoSQL Abstraction Adapter
pub struct InMemoryUserRepository {
    users: std::sync::Arc<tokio::sync::Mutex<Vec<UserEntity>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        let default_admin = UserEntity {
            id: "usr_admin_01".to_string(),
            email: "admin@ferrox.dev".to_string(),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$fakehash".to_string(),
            full_name: "Ferrox Admin".to_string(),
            role: "admin".to_string(),
            totp_enabled: false,
            totp_secret: None,
            onboarding_completed: true,
            avatar_url: Some("https://ferrox-rust.dev/logo.jpg".to_string()),
            created_at: "2026-09-10T12:00:00Z".to_string(),
        };

        Self {
            users: std::sync::Arc::new(tokio::sync::Mutex::new(vec![default_admin])),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<UserEntity>, anyhow::Error> {
        let guard = self.users.lock().await;
        Ok(guard.iter().find(|u| u.id == id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, anyhow::Error> {
        let guard = self.users.lock().await;
        Ok(guard.iter().find(|u| u.email.eq_ignore_ascii_case(email)).cloned())
    }

    async fn save(&self, user: &UserEntity) -> Result<(), anyhow::Error> {
        let mut guard = self.users.lock().await;
        if let Some(pos) = guard.iter().position(|u| u.id == user.id) {
            guard[pos] = user.clone();
        } else {
            guard.push(user.clone());
        }
        Ok(())
    }

    async fn list_users(&self, _limit: usize, _offset: usize) -> Result<Vec<UserEntity>, anyhow::Error> {
        let guard = self.users.lock().await;
        Ok(guard.clone())
    }

    async fn count_users(&self) -> Result<u64, anyhow::Error> {
        let guard = self.users.lock().await;
        Ok(guard.len() as u64)
    }
}
