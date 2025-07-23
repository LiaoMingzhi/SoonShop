// 微服务集成测试模块
// 用于测试各服务间的交互和端到端流程

pub mod common;
pub mod api_gateway_tests;
pub mod user_service_tests;
pub mod product_service_tests;
pub mod order_service_tests;
pub mod payment_service_tests;
pub mod inventory_service_tests;
pub mod notification_service_tests;
pub mod e2e_tests;

// 重新导出测试工具
pub use common::*; 