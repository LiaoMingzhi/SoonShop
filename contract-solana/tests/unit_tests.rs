/**
 * SoonShop核心智能合约单元测试
 * 
 * 针对各个模块的独立功能进行单元测试，包括：
 * - 工具函数测试
 * - 数据结构验证
 * - 计算逻辑验证
 * - 常量和配置测试
 */

use anchor_lang::prelude::*;
use soonshop_core::{constants::*, state::*, utils::*};

// ================================
// 工具函数单元测试
// ================================

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_calculate_multiplier_reward() {
        // 测试基本倍增奖励计算
        let base_amount = 1000000; // 1.0 USDC
        let multiplier = 3;
        let quality_score = 8;

        let reward = calculate_multiplier_reward(base_amount, multiplier, quality_score);
        
        // 期望: 1000000 * 3 * (8/10) = 2400000
        assert_eq!(reward, 2400000);
    }

    #[test]
    fn test_calculate_multiplier_reward_edge_cases() {
        // 测试边界条件
        
        // 最小值测试
        let reward_min = calculate_multiplier_reward(1, MIN_MULTIPLIER, MIN_QUALITY_SCORE);
        assert!(reward_min > 0);

        // 最大值测试
        let reward_max = calculate_multiplier_reward(
            u64::MAX / 100, // 避免溢出
            MAX_MULTIPLIER, 
            MAX_QUALITY_SCORE
        );
        assert!(reward_max > 0);

        // 零值测试
        let reward_zero = calculate_multiplier_reward(0, 5, 8);
        assert_eq!(reward_zero, 0);
    }

    #[test]
    fn test_calculate_chain_multiplier() {
        let original_amount = 5000000; // 5.0 USDC
        let chain_level = 2;
        let supply_ratio = 0.3; // 30%

        let chain_reward = calculate_chain_multiplier(original_amount, chain_level, supply_ratio);
        
        // 期望: 5000000 * 0.3 * (0.9^2) = 5000000 * 0.3 * 0.81 = 1215000
        assert_eq!(chain_reward, 1215000);
    }

    #[test]
    fn test_validate_username() {
        // 有效用户名
        assert!(validate_username("alice"));
        assert!(validate_username("bob123"));
        assert!(validate_username("user_name"));

        // 无效用户名
        assert!(!validate_username("ab")); // 太短
        assert!(!validate_username(&"a".repeat(MAX_USERNAME_LENGTH + 1))); // 太长
        assert!(!validate_username("user@name")); // 包含特殊字符
        assert!(!validate_username("")); // 空字符串
    }

    #[test]
    fn test_validate_email() {
        // 有效邮箱
        assert!(validate_email("user@example.com"));
        assert!(validate_email("test.email+tag@domain.co.uk"));

        // 无效邮箱
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("user@"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email(""));
    }

    #[test]
    fn test_validate_url() {
        // 有效URL
        assert!(validate_url("https://example.com"));
        assert!(validate_url("http://localhost:3000"));
        assert!(validate_url("https://sub.domain.com/path?param=value"));

        // 无效URL
        assert!(!validate_url("not-a-url"));
        assert!(!validate_url("ftp://example.com")); // 不支持的协议
        assert!(!validate_url(""));
    }

    #[test]
    fn test_calculate_platform_fee() {
        let amount = 1000000; // 1.0 USDC
        let fee_rate_bp = 50; // 0.5%

        let fee = calculate_platform_fee(amount, fee_rate_bp);
        
        // 期望: 1000000 * 50 / 10000 = 5000
        assert_eq!(fee, 5000);
    }

    #[test]
    fn test_is_timestamp_valid() {
        let current_time = 1640995200; // 2022-01-01 00:00:00

        // 有效时间戳
        assert!(is_timestamp_valid(current_time, current_time));
        assert!(is_timestamp_valid(current_time - 3600, current_time)); // 1小时前

        // 无效时间戳
        assert!(!is_timestamp_valid(current_time + 3600, current_time)); // 未来时间
        assert!(!is_timestamp_valid(0, current_time)); // 过于久远
    }

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(1000000), "1.000000");
        assert_eq!(format_price(1500000), "1.500000");
        assert_eq!(format_price(123), "0.000123");
    }

    #[test]
    fn test_parse_price() {
        assert_eq!(parse_price("1.000000").unwrap(), 1000000);
        assert_eq!(parse_price("1.5").unwrap(), 1500000);
        assert_eq!(parse_price("0.000123").unwrap(), 123);

        // 无效格式
        assert!(parse_price("invalid").is_err());
        assert!(parse_price("").is_err());
    }
}

// ================================
// 数据结构单元测试
// ================================

#[cfg(test)]
mod state_tests {
    use super::*;

    #[test]
    fn test_platform_config_creation() {
        let config = PlatformConfig {
            authority: Pubkey::new_unique(),
            base_multiplier: DEFAULT_BASE_MULTIPLIER,
            max_multiplier: MAX_MULTIPLIER,
            platform_fee_rate: DEFAULT_PLATFORM_FEE_RATE,
            reward_pool: Pubkey::new_unique(),
            emergency_authority: Pubkey::new_unique(),
            upgrade_authority: Pubkey::new_unique(),
            is_paused: false,
            is_emergency_paused: false,
            created_at: 1640995200,
            last_updated: 1640995200,
            total_rewards_distributed: 0,
            total_vouchers_issued: 0,
            total_enterprises: 0,
            total_transactions: 0,
            total_volume: 0,
            admins: vec![],
            max_admins: MAX_ADMINS as u8,
        };

        assert_eq!(config.base_multiplier, DEFAULT_BASE_MULTIPLIER);
        assert!(!config.is_paused);
        assert_eq!(config.total_rewards_distributed, 0);
    }

    #[test]
    fn test_user_account_creation() {
        let user_account = UserAccount {
            user_id: Pubkey::new_unique(),
            user_type: UserType::Consumer,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            phone: "+1234567890".to_string(),
            address: "123 Test St".to_string(),
            reputation_score: 75,
            is_verified: true,
            is_active: true,
            created_at: 1640995200,
            last_active: 1640995200,
            total_transactions: 0,
            total_volume: 0,
            profile_image_url: None,
            bio: None,
            website: None,
            social_links: vec![],
            preferences: UserPreferences::default(),
            statistics: UserStatistics::default(),
        };

        assert_eq!(user_account.user_type, UserType::Consumer);
        assert!(user_account.is_verified);
        assert_eq!(user_account.reputation_score, 75);
    }

    #[test]
    fn test_voucher_creation() {
        let voucher = Voucher {
            id: "test_voucher".to_string(),
            producer: Pubkey::new_unique(),
            product_info: ProductInfo {
                name: "Test Product".to_string(),
                description: "A test product".to_string(),
                category: "Test".to_string(),
                price: 1000000,
                unit: "piece".to_string(),
                specifications: "1kg".to_string(),
                origin: "Local".to_string(),
                expiry_days: 30,
            },
            voucher_config: VoucherConfig {
                voucher_type: VoucherType::Basic,
                quantity: 100,
                price_per_unit: 1000000,
                validity_period: DEFAULT_VOUCHER_VALIDITY,
                transferable: false,
                stackable: true,
            },
            total_quantity: 100,
            claimed_quantity: 0,
            consumed_quantity: 0,
            distribution_rules: DistributionRules::default(),
            quality_assurance: QualityAssurance::default(),
            status: VoucherStatus::Active,
            created_at: 1640995200,
            expires_at: Some(1640995200 + DEFAULT_VOUCHER_VALIDITY),
            last_updated: 1640995200,
        };

        assert_eq!(voucher.status, VoucherStatus::Active);
        assert_eq!(voucher.claimed_quantity, 0);
        assert_eq!(voucher.total_quantity, 100);
    }

    #[test]
    fn test_evaluation_scores_calculation() {
        let scores = EvaluationScores {
            production_capacity: 85,
            product_quality: 90,
            service_level: 80,
            social_responsibility: 75,
            innovation_capability: 88,
        };

        let total_score = scores.calculate_total_score();
        let weighted_score = scores.calculate_weighted_score();

        // 总分应该是平均值
        assert_eq!(total_score, (85 + 90 + 80 + 75 + 88) / 5);
        
        // 加权分数应该考虑不同权重
        assert!(weighted_score > 0);
        assert!(weighted_score <= 100);
    }
}

// ================================
// 常量验证测试
// ================================

#[cfg(test)]
mod constants_tests {
    use super::*;

    #[test]
    fn test_multiplier_constants() {
        assert!(MIN_MULTIPLIER > 0);
        assert!(MAX_MULTIPLIER >= MIN_MULTIPLIER);
        assert!(DEFAULT_BASE_MULTIPLIER >= MIN_MULTIPLIER);
        assert!(DEFAULT_BASE_MULTIPLIER <= MAX_MULTIPLIER);
    }

    #[test]
    fn test_fee_constants() {
        assert!(DEFAULT_PLATFORM_FEE_RATE <= MAX_PLATFORM_FEE_RATE);
        assert!(MAX_PLATFORM_FEE_RATE <= BASIS_POINTS_BASE);
    }

    #[test]
    fn test_time_constants() {
        assert_eq!(SECONDS_PER_DAY, 24 * 60 * 60);
        assert_eq!(SECONDS_PER_WEEK, 7 * SECONDS_PER_DAY);
        assert_eq!(SECONDS_PER_MONTH, 30 * SECONDS_PER_DAY);
        assert_eq!(SECONDS_PER_YEAR, 365 * SECONDS_PER_DAY);
    }

    #[test]
    fn test_size_constants() {
        assert_eq!(PUBKEY_SIZE, 32);
        assert_eq!(BOOL_SIZE, 1);
        assert_eq!(U64_SIZE, 8);
        assert_eq!(I64_SIZE, 8);
    }

    #[test]
    fn test_limit_constants() {
        assert!(MAX_USERNAME_LENGTH >= MIN_USERNAME_LENGTH);
        assert!(MAX_STRING_LENGTH > 0);
        assert!(MAX_VOUCHER_CLAIM_QUANTITY > 0);
        assert!(MAX_ADMINS > 0);
    }

    #[test]
    fn test_evaluation_constants() {
        assert!(MIN_EVALUATION_SCORE <= MAX_EVALUATION_SCORE);
        assert!(EVALUATION_PASS_THRESHOLD <= MAX_EVALUATION_SCORE);
        assert!(EVALUATION_EXCELLENT_THRESHOLD <= MAX_EVALUATION_SCORE);
        assert!(MIN_EVALUATION_INTERVAL > 0);
        assert!(EVALUATION_VALIDITY_PERIOD > MIN_EVALUATION_INTERVAL);
    }
}

// ================================
// 数学计算测试
// ================================

#[cfg(test)]
mod math_tests {
    use super::*;

    #[test]
    fn test_percentage_calculations() {
        // 测试百分比计算
        assert_eq!(calculate_percentage(1000, 50), 500); // 50%
        assert_eq!(calculate_percentage(1000, 25), 250); // 25%
        assert_eq!(calculate_percentage(1000, 100), 1000); // 100%
        assert_eq!(calculate_percentage(1000, 0), 0); // 0%
    }

    #[test]
    fn test_basis_points_calculations() {
        // 测试基点计算
        assert_eq!(calculate_basis_points(10000, 100), 100); // 1%
        assert_eq!(calculate_basis_points(10000, 50), 50); // 0.5%
        assert_eq!(calculate_basis_points(10000, 10000), 10000); // 100%
    }

    #[test]
    fn test_safe_math_operations() {
        // 测试安全数学运算
        
        // 安全加法
        assert_eq!(safe_add(100, 200).unwrap(), 300);
        assert!(safe_add(u64::MAX, 1).is_err()); // 溢出
        
        // 安全乘法
        assert_eq!(safe_mul(100, 200).unwrap(), 20000);
        assert!(safe_mul(u64::MAX, 2).is_err()); // 溢出
        
        // 安全除法
        assert_eq!(safe_div(100, 2).unwrap(), 50);
        assert!(safe_div(100, 0).is_err()); // 除零
    }

    #[test]
    fn test_compound_calculations() {
        // 测试复合计算
        let principal = 1000000; // 1.0 USDC
        let rate = 10; // 10%
        let periods = 3;

        let compound_amount = calculate_compound_reward(principal, rate, periods);
        
        // 1000000 * (1.1)^3 = 1331000
        assert_eq!(compound_amount, 1331000);
    }
}

// ================================
// 验证函数测试
// ================================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_price_validation() {
        // 有效价格
        assert!(validate_price(1000000)); // 1.0 USDC
        assert!(validate_price(1)); // 最小价格

        // 无效价格
        assert!(!validate_price(0)); // 零价格
    }

    #[test]
    fn test_quantity_validation() {
        // 有效数量
        assert!(validate_quantity(1));
        assert!(validate_quantity(100));
        assert!(validate_quantity(MAX_VOUCHER_CLAIM_QUANTITY));

        // 无效数量
        assert!(!validate_quantity(0)); // 零数量
        assert!(!validate_quantity(MAX_VOUCHER_CLAIM_QUANTITY + 1)); // 超过限制
    }

    #[test]
    fn test_multiplier_validation() {
        // 有效倍增系数
        assert!(validate_multiplier(MIN_MULTIPLIER));
        assert!(validate_multiplier(DEFAULT_BASE_MULTIPLIER));
        assert!(validate_multiplier(MAX_MULTIPLIER));

        // 无效倍增系数
        assert!(!validate_multiplier(MIN_MULTIPLIER - 1)); // 低于最小值
        assert!(!validate_multiplier(MAX_MULTIPLIER + 1)); // 超过最大值
    }

    #[test]
    fn test_evaluation_score_validation() {
        // 有效评分
        assert!(validate_evaluation_score(MIN_EVALUATION_SCORE));
        assert!(validate_evaluation_score(EVALUATION_PASS_THRESHOLD));
        assert!(validate_evaluation_score(MAX_EVALUATION_SCORE));

        // 无效评分
        assert!(!validate_evaluation_score(MAX_EVALUATION_SCORE + 1)); // 超过最大值
    }

    #[test]
    fn test_pubkey_validation() {
        // 有效公钥
        let valid_pubkey = Pubkey::new_unique();
        assert!(validate_pubkey(&valid_pubkey));

        // 无效公钥（系统程序地址等）
        assert!(!validate_pubkey(&system_program::ID));
        assert!(!validate_pubkey(&Pubkey::default()));
    }
}

// ================================
// 状态转换测试
// ================================

#[cfg(test)]
mod state_transition_tests {
    use super::*;

    #[test]
    fn test_voucher_status_transitions() {
        // 测试提货券状态转换
        let mut voucher = create_test_voucher();

        // 初始状态
        assert_eq!(voucher.status, VoucherStatus::Active);

        // 暂停
        voucher.pause().unwrap();
        assert_eq!(voucher.status, VoucherStatus::Paused);

        // 恢复
        voucher.resume().unwrap();
        assert_eq!(voucher.status, VoucherStatus::Active);

        // 结束
        voucher.end().unwrap();
        assert_eq!(voucher.status, VoucherStatus::Ended);

        // 不能从结束状态恢复
        assert!(voucher.resume().is_err());
    }

    #[test]
    fn test_user_verification_states() {
        let mut user = create_test_user();

        // 初始状态：未验证
        assert!(!user.is_verified);

        // 验证用户
        user.verify().unwrap();
        assert!(user.is_verified);

        // 暂停用户
        user.suspend().unwrap();
        assert!(!user.is_active);

        // 恢复用户
        user.activate().unwrap();
        assert!(user.is_active);
    }

    #[test]
    fn test_platform_pause_states() {
        let mut platform = create_test_platform_config();

        // 初始状态：正常运行
        assert!(!platform.is_paused);
        assert!(!platform.is_emergency_paused);

        // 常规暂停
        platform.pause().unwrap();
        assert!(platform.is_paused);

        // 紧急暂停
        platform.emergency_pause().unwrap();
        assert!(platform.is_emergency_paused);
        assert!(platform.is_paused);

        // 恢复
        platform.resume().unwrap();
        assert!(!platform.is_emergency_paused);
        assert!(!platform.is_paused);
    }

    fn create_test_voucher() -> Voucher {
        Voucher {
            id: "test".to_string(),
            producer: Pubkey::new_unique(),
            product_info: ProductInfo::default(),
            voucher_config: VoucherConfig::default(),
            total_quantity: 100,
            claimed_quantity: 0,
            consumed_quantity: 0,
            distribution_rules: DistributionRules::default(),
            quality_assurance: QualityAssurance::default(),
            status: VoucherStatus::Active,
            created_at: 1640995200,
            expires_at: None,
            last_updated: 1640995200,
        }
    }

    fn create_test_user() -> UserAccount {
        UserAccount::default()
    }

    fn create_test_platform_config() -> PlatformConfig {
        PlatformConfig::default()
    }
}

// ================================
// 辅助函数
// ================================

// 这些函数可能需要在实际的utils.rs中实现
fn calculate_percentage(value: u64, percentage: u8) -> u64 {
    value * percentage as u64 / 100
}

fn calculate_basis_points(value: u64, bp: u16) -> u64 {
    value * bp as u64 / BASIS_POINTS_BASE as u64
}

fn safe_add(a: u64, b: u64) -> Result<u64, &'static str> {
    a.checked_add(b).ok_or("Addition overflow")
}

fn safe_mul(a: u64, b: u64) -> Result<u64, &'static str> {
    a.checked_mul(b).ok_or("Multiplication overflow")
}

fn safe_div(a: u64, b: u64) -> Result<u64, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn calculate_compound_reward(principal: u64, rate: u8, periods: u8) -> u64 {
    let rate_factor = 100 + rate as u64;
    let mut result = principal;
    
    for _ in 0..periods {
        result = result * rate_factor / 100;
    }
    
    result
}

fn validate_price(price: u64) -> bool {
    price > 0
}

fn validate_quantity(quantity: u32) -> bool {
    quantity > 0 && quantity <= MAX_VOUCHER_CLAIM_QUANTITY
}

fn validate_multiplier(multiplier: u8) -> bool {
    multiplier >= MIN_MULTIPLIER && multiplier <= MAX_MULTIPLIER
}

fn validate_evaluation_score(score: u8) -> bool {
    score <= MAX_EVALUATION_SCORE
}

fn validate_pubkey(pubkey: &Pubkey) -> bool {
    *pubkey != Pubkey::default() && *pubkey != system_program::ID
} 