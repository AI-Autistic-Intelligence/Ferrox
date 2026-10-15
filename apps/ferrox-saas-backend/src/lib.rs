pub mod config;
pub mod modules {
    pub mod admin;
    pub mod auth;
    pub mod docs;
    #[cfg(feature = "founder-suite")]
    pub mod founder;
    pub mod health;
    pub mod mailer;
    pub mod notifications;
    pub mod observability;
    pub mod users;
}
pub mod persistence;
