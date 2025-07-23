/**
 * SoonShop核心智能合约状态模块
 * 
 * 本模块导出平台管理、钱包、提货券相关的状态结构体和枚举
 */

pub mod platform;
pub mod wallet;
pub mod voucher;

// 重新导出主要类型
pub use platform::*;
pub use wallet::*;
pub use voucher::*; 