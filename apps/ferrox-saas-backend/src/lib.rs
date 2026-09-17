pub mod config;
pub mod modules {
    pub mod admin;
    pub mod auth;
    #[cfg(feature = "founder-suite")]
    pub mod founder;
    pub mod mailer;
    pub mod notifications;
    pub mod observability;
    pub mod users;
}
pub mod persistence;
