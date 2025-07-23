// 日志相关工具 - 基本实现
use tracing::Level;

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
} 