use sea_orm::{Database, DatabaseConnection, ConnectOptions, DbErr, TransactionTrait};
use std::time::Duration;
use tracing::log;

pub struct DatabaseManager {
    pub connection: DatabaseConnection,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let mut opt = ConnectOptions::new(database_url.to_string());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let connection = Database::connect(opt).await?;
        
        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    Connection(#[from] DbErr),
    #[error("Migration error: {0}")]
    Migration(String),
    #[error("Query error: {0}")]
    Query(String),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

// 分页查询helper
pub struct PaginationParams {
    pub page: u64,
    pub limit: u64,
}

impl PaginationParams {
    pub fn new(page: Option<u64>, limit: Option<u64>) -> Self {
        Self {
            page: page.unwrap_or(1),
            limit: limit.unwrap_or(20).min(100), // 最大100条
        }
    }

    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.limit
    }
}

// 使用 SeaORM 的事务 helper
pub async fn with_transaction<F, T, E>(db: &DatabaseConnection, f: F) -> Result<T, E>
where
    F: FnOnce(&sea_orm::DatabaseTransaction) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send + '_>>,
    E: From<DbErr>,
{
    let txn = db.begin().await?;
    let result = f(&txn).await;
    
    match result {
        Ok(value) => {
            txn.commit().await?;
            Ok(value)
        }
        Err(e) => {
            txn.rollback().await?;
            Err(e)
        }
    }
} 