// 增强事件系统使用示例
// 文件路径: /d:/workspace/Solana/SoonShop/backend-microservices/shared/events/examples/enhanced_event_system.rs

use events::{
    EventSystem, EventSystemConfig, EventHandler, RetryConfig, RetryStrategy,
    ErrorInfo, ErrorType, InMemoryEventMetrics, InMemoryErrorStorage,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::Duration;
use std::sync::Arc;
use async_trait::async_trait;

/// 示例事件数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserRegisteredEvent {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub registration_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderCreatedEvent {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub total_amount: i64,
    pub items: Vec<OrderItem>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderItem {
    pub product_id: Uuid,
    pub quantity: u32,
    pub price: i64,
}

/// 用户注册事件处理器
struct UserRegistrationHandler;

#[async_trait]
impl EventHandler<UserRegisteredEvent> for UserRegistrationHandler {
    async fn handle(&self, data: &UserRegisteredEvent) -> anyhow::Result<()> {
        println!("🎉 处理用户注册事件:");
        println!("  用户ID: {}", data.user_id);
        println!("  邮箱: {}", data.email);
        println!("  用户名: {}", data.username);
        
        // 模拟处理时间
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // 模拟偶发错误（20%概率）
        if rand::random::<f64>() < 0.2 {
            return Err(anyhow::anyhow!("模拟的临时网络错误"));
        }
        
        println!("✅ 用户注册事件处理完成");
        Ok(())
    }
}

/// 订单创建事件处理器
struct OrderCreationHandler;

#[async_trait]
impl EventHandler<OrderCreatedEvent> for OrderCreationHandler {
    async fn handle(&self, data: &OrderCreatedEvent) -> anyhow::Result<()> {
        println!("📦 处理订单创建事件:");
        println!("  订单ID: {}", data.order_id);
        println!("  用户ID: {}", data.user_id);
        println!("  总金额: ${:.2}", data.total_amount as f64 / 100.0);
        println!("  商品数量: {}", data.items.len());
        
        // 模拟处理时间
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        // 模拟业务逻辑错误（10%概率）
        if rand::random::<f64>() < 0.1 {
            return Err(anyhow::anyhow!("库存不足"));
        }
        
        println!("✅ 订单创建事件处理完成");
        Ok(())
    }
}

/// 始终失败的事件处理器（用于演示重试机制）
struct FailingHandler;

#[async_trait]
impl EventHandler<UserRegisteredEvent> for FailingHandler {
    async fn handle(&self, data: &UserRegisteredEvent) -> anyhow::Result<()> {
        println!("❌ 故意失败的处理器: {}", data.user_id);
        Err(anyhow::anyhow!("模拟的持续失败"))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    println!("🚀 启动增强事件系统示例");
    println!("=====================================");
    
    // 1. 配置事件系统
    let config = EventSystemConfig {
        rabbitmq_url: "amqp://soonshop:soonshop123@localhost:5672".to_string(),
        exchange_name: "soonshop.events.demo".to_string(),
        queue_prefix: "demo".to_string(),
        retry_config: RetryConfig::exponential_backoff(
            Duration::from_secs(1),    // 初始延迟
            3,                         // 最大重试次数
            2.0,                       // 退避因子
            Duration::from_secs(10),   // 最大延迟
        ),
        enable_metrics: true,
        enable_error_storage: true,
        default_source: "demo-service".to_string(),
    };
    
    // 2. 创建并初始化事件系统
    let mut event_system = EventSystem::new(config);
    
    println!("🔧 初始化事件系统...");
    if let Err(e) = event_system.initialize().await {
        println!("⚠️  事件系统初始化失败（可能是RabbitMQ未启动）: {}", e);
        println!("   继续使用模拟模式演示功能...");
        return demonstrate_offline_features().await;
    }
    
    println!("✅ 事件系统初始化成功");
    
    // 3. 注册事件处理器
    println!("📝 注册事件处理器...");
    
    event_system.subscribe("user.registered", UserRegistrationHandler).await?;
    event_system.subscribe("order.created", OrderCreationHandler).await?;
    event_system.subscribe("user.registered.failing", FailingHandler).await?;
    
    println!("✅ 事件处理器注册完成");
    
    // 4. 发布测试事件
    println!("\n📢 开始发布测试事件...");
    
    // 发布用户注册事件
    for i in 1..=5 {
        let user_event = UserRegisteredEvent {
            user_id: Uuid::new_v4(),
            email: format!("user{}@example.com", i),
            username: format!("user{}", i),
            registration_time: chrono::Utc::now(),
        };
        
        println!("📤 发布用户注册事件 #{}", i);
        event_system.publish(
            "user.registered".to_string(),
            "user.registered",
            user_event,
        ).await?;
        
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // 发布订单创建事件
    for i in 1..=3 {
        let order_event = OrderCreatedEvent {
            order_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            total_amount: (i as i64) * 5000, // $50.00, $100.00, $150.00
            items: vec![
                OrderItem {
                    product_id: Uuid::new_v4(),
                    quantity: i,
                    price: 5000,
                }
            ],
            created_at: chrono::Utc::now(),
        };
        
        println!("📤 发布订单创建事件 #{}", i);
        event_system.publish(
            "order.created".to_string(),
            "order.created",
            order_event,
        ).await?;
        
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // 发布故意失败的事件（演示重试机制）
    println!("📤 发布故意失败的事件（演示重试机制）");
    let failing_event = UserRegisteredEvent {
        user_id: Uuid::new_v4(),
        email: "failing@example.com".to_string(),
        username: "failing_user".to_string(),
        registration_time: chrono::Utc::now(),
    };
    
    if let Err(e) = event_system.publish(
        "user.registered".to_string(),
        "user.registered.failing",
        failing_event,
    ).await {
        println!("⚠️  预期的失败: {}", e);
    }
    
    // 5. 等待事件处理
    println!("\n⏳ 等待事件处理完成...");
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    // 6. 显示指标和统计信息
    println!("\n📊 显示系统指标...");
    display_metrics(&event_system).await?;
    
    // 7. 显示错误信息
    println!("\n🔍 显示错误信息...");
    display_errors(&event_system).await?;
    
    // 8. 健康检查
    println!("\n🏥 执行健康检查...");
    let health = event_system.health_check().await?;
    println!("系统健康状态: {}", if health.is_healthy() { "✅ 健康" } else { "❌ 异常" });
    println!("发布器可用: {}", health.publisher_available);
    println!("订阅器可用: {}", health.subscriber_available);
    println!("指标收集可用: {}", health.metrics_available);
    println!("错误存储可用: {}", health.error_storage_available);
    println!("RabbitMQ连接: {}", health.rabbitmq_connected);
    if let Some(summary) = &health.metrics_summary {
        println!("指标摘要: {}", summary);
    }
    
    println!("\n🎉 增强事件系统示例完成！");
    Ok(())
}

/// 显示指标信息
async fn display_metrics(event_system: &EventSystem) -> anyhow::Result<()> {
    if let Some(metrics) = event_system.metrics() {
        let stats = metrics.get_stats(None).await;
        
        println!("📈 事件指标统计:");
        println!("  成功发布: {}", stats.successful_publishes);
        println!("  失败发布: {}", stats.failed_publishes);
        println!("  重试次数: {}", stats.retry_attempts);
        println!("  死信消息: {}", stats.dead_letter_messages);
        println!("  平均处理时间: {:.2}ms", stats.avg_processing_time);
        
        if !stats.error_type_counts.is_empty() {
            println!("  错误类型统计:");
            for (error_type, count) in &stats.error_type_counts {
                println!("    {:?}: {}", error_type, count);
            }
        }
        
        let metric_data = metrics.get_metrics().await;
        println!("  总指标数量: {}", metric_data.len());
    } else {
        println!("❌ 指标收集器不可用");
    }
    
    Ok(())
}

/// 显示错误信息
async fn display_errors(event_system: &EventSystem) -> anyhow::Result<()> {
    if let Some(error_storage) = event_system.error_storage() {
        let query = events::ErrorQuery {
            resolved: Some(false), // 只显示未解决的错误
            limit: Some(10),
            ..Default::default()
        };
        
        let errors = error_storage.query_errors(&query).await?;
        
        println!("🚨 未解决的错误 ({}):", errors.len());
        for error in errors {
            println!("  错误ID: {}", error.id);
            println!("  事件ID: {}", error.event_id);
            println!("  错误类型: {:?}", error.error_info.error_type);
            println!("  错误消息: {}", error.error_info.message);
            println!("  发生时间: {}", error.error_info.occurred_at.format("%Y-%m-%d %H:%M:%S"));
            if let Some(retry_state) = &error.retry_state {
                println!("  重试次数: {}", retry_state.attempt);
                println!("  重试耗尽: {}", retry_state.exhausted);
            }
            println!("  ---");
        }
        
        // 显示统计信息
        let stats = error_storage.get_statistics(None).await?;
        println!("📊 错误统计:");
        println!("  总错误数: {}", stats.total_errors);
        println!("  未解决错误数: {}", stats.unresolved_errors);
    } else {
        println!("❌ 错误存储不可用");
    }
    
    Ok(())
}

/// 离线功能演示（当RabbitMQ不可用时）
async fn demonstrate_offline_features() -> anyhow::Result<()> {
    println!("\n🔧 演示离线功能...");
    
    // 1. 演示重试策略
    println!("\n📋 重试策略演示:");
    let strategies = vec![
        ("固定延迟", RetryStrategy::FixedDelay {
            delay: Duration::from_secs(1),
            max_attempts: 3,
        }),
        ("指数退避", RetryStrategy::ExponentialBackoff {
            initial_delay: Duration::from_secs(1),
            backoff_factor: 2.0,
            max_delay: Duration::from_secs(30),
            max_attempts: 5,
        }),
        ("自定义延迟", RetryStrategy::CustomDelays {
            delays: vec![
                Duration::from_secs(1),
                Duration::from_secs(3),
                Duration::from_secs(5),
                Duration::from_secs(10),
            ],
        }),
    ];
    
    for (name, strategy) in strategies {
        println!("  {} 策略:", name);
        for attempt in 0..6 {
            if let Some(delay) = strategy.calculate_delay(attempt) {
                println!("    尝试 {}: 延迟 {:?}", attempt, delay);
            } else {
                println!("    尝试 {}: 不再重试", attempt);
                break;
            }
        }
    }
    
    // 2. 演示错误分类
    println!("\n🏷️  错误分类演示:");
    let error_types = vec![
        ErrorType::Transient,
        ErrorType::Permanent,
        ErrorType::Network,
        ErrorType::Serialization,
        ErrorType::Business,
        ErrorType::System,
        ErrorType::Unknown,
    ];
    
    for error_type in error_types {
        println!("  {:?}: 可重试 = {}", error_type, error_type.is_retryable());
    }
    
    // 3. 演示内存指标收集
    println!("\n📊 内存指标收集演示:");
    let metrics = Arc::new(InMemoryEventMetrics::new());
    
    // 模拟一些指标
    metrics.record_successful_publish("test.exchange", "test.key").await;
    metrics.record_successful_publish("test.exchange", "test.key").await;
    metrics.record_failed_publish("test.exchange", "test.key", &ErrorType::Network).await;
    metrics.record_retry_attempt("test.exchange", "test.key", 1).await;
    metrics.record_processing_time("test.exchange", "test.key", 150.0).await;
    
    let stats = metrics.get_stats(Some("test.exchange")).await;
    println!("  成功: {}", stats.successful_publishes);
    println!("  失败: {}", stats.failed_publishes);
    println!("  重试: {}", stats.retry_attempts);
    println!("  平均处理时间: {:.2}ms", stats.avg_processing_time);
    
    // 4. 演示内存错误存储
    println!("\n💾 内存错误存储演示:");
    let error_storage = Arc::new(InMemoryErrorStorage::new());
    
    let event_id = Uuid::new_v4();
    let error_info = ErrorInfo::new(
        ErrorType::Network,
        "连接超时".to_string(),
    ).with_details("无法连接到外部服务".to_string());
    
    let record_id = error_storage.store_error(event_id, &error_info).await?;
    println!("  存储了错误记录: {}", record_id);
    
    let record = error_storage.get_error(record_id).await?.unwrap();
    println!("  错误消息: {}", record.error_info.message);
    println!("  错误类型: {:?}", record.error_info.error_type);
    
    println!("\n✅ 离线功能演示完成");
    Ok(())
} 