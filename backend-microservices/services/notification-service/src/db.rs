// 数据库连接模块

use sqlx::PgPool;

// Database类型别名，用于API处理器
pub type Database = PgPool;

pub async fn connect_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
} 