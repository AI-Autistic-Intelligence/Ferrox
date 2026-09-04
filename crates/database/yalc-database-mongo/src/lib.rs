use mongodb::{Client, options::ClientOptions};
use yalc_errors::AppError;
use std::time::Duration;

#[derive(Clone)]
pub struct MongoClient {
    pub client: Client,
    pub db_name: String,
}

impl MongoClient {
    pub async fn connect(connection_string: &str, db_name: &str) -> Result<Self, AppError> {
        let mut client_options = ClientOptions::parse(connection_string)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Mongo URL Parse Error: {}", e)))?;

        // Optional: configure pooling explicitly if needed, but defaults are usually fine
        client_options.connect_timeout = Some(Duration::from_secs(10));
        client_options.max_pool_size = Some(100);
        client_options.min_pool_size = Some(5);

        let client = Client::with_options(client_options)
            .map_err(|e| AppError::DatabaseError(format!("Mongo Client Init Error: {}", e)))?;

        // Test connection
        client
            .database(db_name)
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await
            .map_err(|e| AppError::DatabaseError(format!("Mongo Ping Failed: {}", e)))?;

        tracing::info!("Connected to MongoDB: {}", db_name);

        Ok(Self {
            client,
            db_name: db_name.to_string(),
        })
    }
}
