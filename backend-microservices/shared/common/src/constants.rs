// 服务名称
pub const USER_SERVICE: &str = "user-service";
pub const PRODUCT_SERVICE: &str = "product-service";
pub const ORDER_SERVICE: &str = "order-service";
pub const PAYMENT_SERVICE: &str = "payment-service";
pub const INVENTORY_SERVICE: &str = "inventory-service";
pub const NOTIFICATION_SERVICE: &str = "notification-service";
pub const ANALYTICS_SERVICE: &str = "analytics-service";
pub const API_GATEWAY: &str = "api-gateway";

// 默认端口
pub const DEFAULT_USER_SERVICE_PORT: u16 = 8001;
pub const DEFAULT_PRODUCT_SERVICE_PORT: u16 = 8002;
pub const DEFAULT_ORDER_SERVICE_PORT: u16 = 8003;
pub const DEFAULT_PAYMENT_SERVICE_PORT: u16 = 8004;
pub const DEFAULT_INVENTORY_SERVICE_PORT: u16 = 8005;
pub const DEFAULT_NOTIFICATION_SERVICE_PORT: u16 = 8006;
pub const DEFAULT_ANALYTICS_SERVICE_PORT: u16 = 8007;
pub const DEFAULT_GATEWAY_PORT: u16 = 8000;

// 数据库相关
pub const DEFAULT_DB_MAX_CONNECTIONS: u32 = 10;
pub const DEFAULT_DB_MIN_CONNECTIONS: u32 = 1;
pub const DEFAULT_DB_CONNECT_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_DB_IDLE_TIMEOUT_SECS: u64 = 600;

// Redis相关
pub const DEFAULT_REDIS_URL: &str = "redis://localhost:6379";
pub const DEFAULT_REDIS_MAX_CONNECTIONS: u32 = 10;
pub const DEFAULT_REDIS_CONNECTION_TIMEOUT_SECS: u64 = 5;

// JWT相关
pub const DEFAULT_JWT_EXPIRES_IN_SECS: u64 = 3600; // 1小时
pub const DEFAULT_REFRESH_TOKEN_EXPIRES_IN_SECS: u64 = 604800; // 7天

// 分页相关
pub const DEFAULT_PAGE_SIZE: u64 = 20;
pub const MAX_PAGE_SIZE: u64 = 100;

// 消息队列相关
pub const DEFAULT_RABBITMQ_URL: &str = "amqp://localhost:5672";
pub const DEFAULT_RABBITMQ_EXCHANGE: &str = "soonshop-events";
pub const DEFAULT_RABBITMQ_QUEUE_PREFIX: &str = "soonshop";
pub const USER_EVENTS_TOPIC: &str = "user-events";
pub const PRODUCT_EVENTS_TOPIC: &str = "product-events";
pub const ORDER_EVENTS_TOPIC: &str = "order-events";
pub const PAYMENT_EVENTS_TOPIC: &str = "payment-events";
pub const INVENTORY_EVENTS_TOPIC: &str = "inventory-events";
pub const NOTIFICATION_EVENTS_TOPIC: &str = "notification-events";

// HTTP相关
pub const DEFAULT_HTTP_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_MAX_RETRIES: u32 = 3;
pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;

// 监控相关
pub const METRICS_NAMESPACE: &str = "soonshop";
pub const HEALTH_CHECK_PATH: &str = "/health";
pub const METRICS_PATH: &str = "/metrics";

// 业务常量
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_USERNAME_LENGTH: usize = 50;
pub const MAX_EMAIL_LENGTH: usize = 255;
pub const MAX_DESCRIPTION_LENGTH: usize = 1000;

// 货币相关
pub const DEFAULT_CURRENCY: &str = "USD";
pub const PRICE_DECIMAL_PLACES: u32 = 2;

// 文件上传相关
pub const MAX_FILE_SIZE_MB: u64 = 10;
pub const ALLOWED_IMAGE_TYPES: &[&str] = &["image/jpeg", "image/png", "image/gif", "image/webp"];

// 缓存相关
pub const DEFAULT_CACHE_TTL_SECS: u64 = 300; // 5分钟
pub const USER_CACHE_TTL_SECS: u64 = 600; // 10分钟
pub const PRODUCT_CACHE_TTL_SECS: u64 = 1800; // 30分钟 