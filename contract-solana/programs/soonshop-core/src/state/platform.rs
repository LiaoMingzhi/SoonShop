/**
 * SoonShop核心智能合约平台状态模块
 * 
 * 本模块定义了平台级别的状态结构体，包括：
 * - 平台配置信息
 * - 系统运行统计
 * - 全局参数设置
 */

use anchor_lang::prelude::*;
use crate::constants::*;

// ================================
// 平台配置账户
// ================================

/**
 * 平台配置信息
 * 
 * 存储平台的全局配置参数和管理信息
 */
#[account]
#[derive(Debug)]
pub struct PlatformConfig {
    /// 平台超级管理员公钥
    pub super_admin: Pubkey,
    
    /// 平台管理员列表
    pub admins: Vec<Pubkey>,
    
    /// 基础倍增系数
    pub base_multiplier: u8,
    
    /// 最大倍增系数
    pub max_multiplier: u8,
    
    /// 最小倍增系数
    pub min_multiplier: u8,
    
    /// 平台费率（基点）
    pub platform_fee_rate: u16,
    
    /// 奖励池账户
    pub reward_pool: Pubkey,
    
    /// 平台状态
    pub status: PlatformStatus,
    
    /// 是否处于紧急暂停状态
    pub is_emergency_paused: bool,
    
    /// 紧急暂停时间
    pub emergency_pause_time: Option<i64>,
    
    /// 今日紧急暂停次数
    pub daily_emergency_pauses: u8,
    
    /// 上次重置紧急暂停计数的日期
    pub last_emergency_reset_day: i64,
    
    /// 平台创建时间
    pub created_at: i64,
    
    /// 最后更新时间
    pub updated_at: i64,
    
    /// 当前版本号
    pub version: String,
    
    /// 系统统计信息
    pub statistics: PlatformStatistics,
}

/**
 * 平台统计信息
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct PlatformStatistics {
    /// 总用户数
    pub total_users: u64,
    
    /// 活跃用户数
    pub active_users: u64,
    
    /// 生产者数量
    pub producer_count: u64,
    
    /// 消费者数量
    pub consumer_count: u64,
    
    /// 评估员数量
    pub evaluator_count: u64,
    
    /// 提货券总数
    pub total_vouchers: u64,
    
    /// 已发布提货券数
    pub issued_vouchers: u64,
    
    /// 已获取提货券数
    pub claimed_vouchers: u64,
    
    /// 已消费提货券数
    pub consumed_vouchers: u64,
    
    /// 总消费金额
    pub total_consumption_amount: u64,
    
    /// 总奖励分发金额
    pub total_rewards_distributed: u64,
    
    /// 平台总收入
    pub total_platform_revenue: u64,
    
    /// 企业评估总数
    pub total_evaluations: u64,
    
    /// 价格监控事件数
    pub price_monitoring_events: u64,
    
    /// 最后统计更新时间
    pub last_stats_update: i64,
}

/**
 * 平台状态枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum PlatformStatus {
    /// 初始化中
    Initializing,
    /// 正常运行
    Active,
    /// 暂停服务
    Paused,
    /// 维护中
    Maintenance,
    /// 紧急状态
    Emergency,
    /// 升级中
    Upgrading,
    /// 已停用
    Deactivated,
}

impl Default for PlatformStatus {
    fn default() -> Self {
        PlatformStatus::Initializing
    }
}

// ================================
// 平台配置实现
// ================================

impl PlatformConfig {
    /// 计算账户所需空间
    pub const SPACE: usize = ACCOUNT_DISCRIMINATOR_SIZE
        + PUBKEY_SIZE  // super_admin
        + VEC_PREFIX_SIZE + MAX_ADMINS * PUBKEY_SIZE  // admins
        + U8_SIZE * 3  // multipliers
        + U16_SIZE     // platform_fee_rate
        + PUBKEY_SIZE  // reward_pool
        + 1            // status enum
        + BOOL_SIZE    // is_emergency_paused
        + OPTION_FLAG_SIZE + I64_SIZE  // emergency_pause_time
        + U8_SIZE      // daily_emergency_pauses
        + I64_SIZE     // last_emergency_reset_day
        + I64_SIZE * 2 // created_at, updated_at
        + STRING_PREFIX_SIZE + 20  // version
        + PlatformStatistics::SPACE; // statistics

    /// 初始化平台配置
    pub fn initialize(
        &mut self,
        super_admin: Pubkey,
        reward_pool: Pubkey,
        base_multiplier: u8,
        platform_fee_rate: u16,
        version: String,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        self.super_admin = super_admin;
        self.admins = Vec::new();
        self.base_multiplier = base_multiplier;
        self.max_multiplier = MAX_MULTIPLIER;
        self.min_multiplier = MIN_MULTIPLIER;
        self.platform_fee_rate = platform_fee_rate;
        self.reward_pool = reward_pool;
        self.status = PlatformStatus::Active;
        self.is_emergency_paused = false;
        self.emergency_pause_time = None;
        self.daily_emergency_pauses = 0;
        self.last_emergency_reset_day = current_time / SECONDS_PER_DAY;
        self.created_at = current_time;
        self.updated_at = current_time;
        self.version = version;
        self.statistics = PlatformStatistics::default();
        
        Ok(())
    }

    /// 添加管理员
    pub fn add_admin(&mut self, admin: Pubkey) -> Result<()> {
        require!(
            self.admins.len() < MAX_ADMINS,
            crate::errors::SoonShopError::TooManyAdmins
        );
        require!(
            !self.admins.contains(&admin),
            crate::errors::SoonShopError::AdminAlreadyExists
        );
        
        self.admins.push(admin);
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 移除管理员
    pub fn remove_admin(&mut self, admin: Pubkey) -> Result<()> {
        let index = self.admins.iter().position(|&x| x == admin)
            .ok_or(crate::errors::SoonShopError::AdminNotFound)?;
        
        self.admins.remove(index);
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 检查是否为超级管理员
    pub fn is_super_admin(&self, pubkey: &Pubkey) -> bool {
        &self.super_admin == pubkey
    }

    /// 检查是否为管理员
    pub fn is_admin(&self, pubkey: &Pubkey) -> bool {
        self.admins.contains(pubkey)
    }

    /// 检查是否有管理权限
    pub fn has_admin_permission(&self, pubkey: &Pubkey) -> bool {
        self.is_super_admin(pubkey) || self.is_admin(pubkey)
    }

    /// 紧急暂停
    pub fn emergency_pause(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        let current_day = current_time / SECONDS_PER_DAY;
        
        // 重置每日计数器
        if current_day > self.last_emergency_reset_day {
            self.daily_emergency_pauses = 0;
            self.last_emergency_reset_day = current_day;
        }
        
        require!(
            self.daily_emergency_pauses < MAX_EMERGENCY_PAUSES_PER_DAY,
            crate::errors::SoonShopError::TooManyEmergencyPauses
        );
        
        self.is_emergency_paused = true;
        self.emergency_pause_time = Some(current_time);
        self.daily_emergency_pauses += 1;
        self.status = PlatformStatus::Emergency;
        self.updated_at = current_time;
        
        Ok(())
    }

    /// 恢复正常运行
    pub fn emergency_resume(&mut self) -> Result<()> {
        self.is_emergency_paused = false;
        self.emergency_pause_time = None;
        self.status = PlatformStatus::Active;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// 更新统计信息
    pub fn update_statistics(&mut self, stats_update: StatisticsUpdate) -> Result<()> {
        let stats = &mut self.statistics;
        
        // 更新各种计数
        stats.total_users = stats_update.total_users.unwrap_or(stats.total_users);
        stats.active_users = stats_update.active_users.unwrap_or(stats.active_users);
        stats.producer_count = stats_update.producer_count.unwrap_or(stats.producer_count);
        stats.consumer_count = stats_update.consumer_count.unwrap_or(stats.consumer_count);
        stats.evaluator_count = stats_update.evaluator_count.unwrap_or(stats.evaluator_count);
        
        // 更新提货券相关统计
        if let Some(vouchers) = stats_update.vouchers_delta {
            stats.total_vouchers = stats.total_vouchers.saturating_add(vouchers);
        }
        if let Some(issued) = stats_update.issued_vouchers_delta {
            stats.issued_vouchers = stats.issued_vouchers.saturating_add(issued);
        }
        if let Some(claimed) = stats_update.claimed_vouchers_delta {
            stats.claimed_vouchers = stats.claimed_vouchers.saturating_add(claimed);
        }
        if let Some(consumed) = stats_update.consumed_vouchers_delta {
            stats.consumed_vouchers = stats.consumed_vouchers.saturating_add(consumed);
        }
        
        // 更新金额统计
        if let Some(amount) = stats_update.consumption_amount_delta {
            stats.total_consumption_amount = stats.total_consumption_amount.saturating_add(amount);
        }
        if let Some(amount) = stats_update.rewards_distributed_delta {
            stats.total_rewards_distributed = stats.total_rewards_distributed.saturating_add(amount);
        }
        if let Some(amount) = stats_update.platform_revenue_delta {
            stats.total_platform_revenue = stats.total_platform_revenue.saturating_add(amount);
        }
        
        stats.last_stats_update = Clock::get()?.unix_timestamp;
        self.updated_at = stats.last_stats_update;
        
        Ok(())
    }
}

impl PlatformStatistics {
    /// 计算账户所需空间
    pub const SPACE: usize = U64_SIZE * 13 + I64_SIZE;
}

/**
 * 统计信息更新结构
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StatisticsUpdate {
    pub total_users: Option<u64>,
    pub active_users: Option<u64>,
    pub producer_count: Option<u64>,
    pub consumer_count: Option<u64>,
    pub evaluator_count: Option<u64>,
    pub vouchers_delta: Option<u64>,
    pub issued_vouchers_delta: Option<u64>,
    pub claimed_vouchers_delta: Option<u64>,
    pub consumed_vouchers_delta: Option<u64>,
    pub consumption_amount_delta: Option<u64>,
    pub rewards_distributed_delta: Option<u64>,
    pub platform_revenue_delta: Option<u64>,
} 