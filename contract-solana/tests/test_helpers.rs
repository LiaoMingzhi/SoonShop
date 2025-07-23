/**
 * SoonShop智能合约测试辅助工具
 * 
 * 提供测试中常用的工具函数和模拟数据，包括：
 * - 模拟数据生成器
 * - 测试环境设置
 * - 断言辅助函数
 * - 时间和日期工具
 * - 随机数据生成
 */

use anchor_lang::prelude::*;
use anchor_spl::token;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::collections::HashMap;

use soonshop_core::{constants::*, state::*, ID as PROGRAM_ID};

// ================================
// 测试数据生成器
// ================================

pub struct MockDataGenerator {
    pub counter: u64,
    pub users: HashMap<String, Keypair>,
}

impl MockDataGenerator {
    pub fn new() -> Self {
        Self {
            counter: 0,
            users: HashMap::new(),
        }
    }

    /// 生成唯一的测试ID
    pub fn generate_id(&mut self, prefix: &str) -> String {
        self.counter += 1;
        format!("{}_{:06}", prefix, self.counter)
    }

    /// 创建测试用户
    pub fn create_test_user(&mut self, user_type: &str) -> Keypair {
        let user = Keypair::new();
        self.users.insert(format!("{}_{}", user_type, self.counter), user);
        self.users.get(&format!("{}_{}", user_type, self.counter)).unwrap().insecure_clone()
    }

    /// 生成测试平台配置
    pub fn create_platform_config(&self) -> PlatformConfig {
        PlatformConfig {
            authority: Pubkey::new_unique(),
            base_multiplier: DEFAULT_BASE_MULTIPLIER,
            max_multiplier: MAX_MULTIPLIER,
            platform_fee_rate: DEFAULT_PLATFORM_FEE_RATE,
            reward_pool: Pubkey::new_unique(),
            emergency_authority: Pubkey::new_unique(),
            upgrade_authority: Pubkey::new_unique(),
            is_paused: false,
            is_emergency_paused: false,
            created_at: get_test_timestamp(),
            last_updated: get_test_timestamp(),
            total_rewards_distributed: 0,
            total_vouchers_issued: 0,
            total_enterprises: 0,
            total_transactions: 0,
            total_volume: 0,
            admins: vec![],
            max_admins: MAX_ADMINS as u8,
        }
    }

    /// 生成测试用户账户
    pub fn create_user_account(&self, user_type: UserType) -> UserAccount {
        UserAccount {
            user_id: Pubkey::new_unique(),
            user_type,
            username: format!("testuser_{}", self.counter),
            email: format!("user{}@test.com", self.counter),
            phone: format!("+1234567{:03}", self.counter % 1000),
            address: format!("{} Test Street", self.counter),
            reputation_score: 75,
            is_verified: true,
            is_active: true,
            created_at: get_test_timestamp(),
            last_active: get_test_timestamp(),
            total_transactions: 0,
            total_volume: 0,
            profile_image_url: None,
            bio: None,
            website: None,
            social_links: vec![],
            preferences: UserPreferences::default(),
            statistics: UserStatistics::default(),
        }
    }

    /// 生成测试提货券
    pub fn create_voucher(&mut self, producer: Pubkey) -> Voucher {
        let voucher_id = self.generate_id("voucher");
        
        Voucher {
            id: voucher_id,
            producer,
            product_info: self.create_product_info(),
            voucher_config: self.create_voucher_config(),
            total_quantity: 1000,
            claimed_quantity: 0,
            consumed_quantity: 0,
            distribution_rules: self.create_distribution_rules(),
            quality_assurance: QualityAssurance::default(),
            status: VoucherStatus::Active,
            created_at: get_test_timestamp(),
            expires_at: Some(get_test_timestamp() + DEFAULT_VOUCHER_VALIDITY),
            last_updated: get_test_timestamp(),
        }
    }

    /// 生成测试商品信息
    pub fn create_product_info(&self) -> ProductInfo {
        ProductInfo {
            name: "测试商品".to_string(),
            description: "这是一个用于测试的商品".to_string(),
            category: "食品".to_string(),
            price: 1000000, // 1.0 USDC
            unit: "份".to_string(),
            specifications: "500g".to_string(),
            origin: "本地生产".to_string(),
            expiry_days: 30,
        }
    }

    /// 生成测试提货券配置
    pub fn create_voucher_config(&self) -> VoucherConfig {
        VoucherConfig {
            voucher_type: VoucherType::Basic,
            quantity: 1000,
            price_per_unit: 1000000,
            validity_period: DEFAULT_VOUCHER_VALIDITY,
            transferable: false,
            stackable: true,
        }
    }

    /// 生成测试分发规则
    pub fn create_distribution_rules(&self) -> DistributionRules {
        DistributionRules {
            max_claim_per_user: 10,
            geographic_restrictions: vec!["北京".to_string(), "上海".to_string()],
            time_restrictions: TimeRestrictions {
                start_time: 0,
                end_time: i64::MAX,
            },
            user_type_restrictions: vec!["consumer".to_string()],
            minimum_reputation_score: 60,
        }
    }

    /// 生成测试企业评估
    pub fn create_evaluation(&mut self, enterprise_id: String, evaluator: Pubkey) -> EnterpriseEvaluation {
        EnterpriseEvaluation {
            id: self.generate_id("evaluation"),
            enterprise_id,
            evaluator,
            evaluation_period: "2023-Q4".to_string(),
            scores: self.create_evaluation_scores(),
            details: self.create_evaluation_details(),
            status: EvaluationStatus::Pending,
            submitted_at: get_test_timestamp(),
            approved_at: None,
            approver: None,
            multiplier_impact: 0,
        }
    }

    /// 生成测试评估分数
    pub fn create_evaluation_scores(&self) -> EvaluationScores {
        EvaluationScores {
            production_capacity: 85,
            product_quality: 90,
            service_level: 80,
            social_responsibility: 75,
            innovation_capability: 88,
        }
    }

    /// 生成测试评估详情
    pub fn create_evaluation_details(&self) -> EvaluationDetails {
        EvaluationDetails {
            evaluation_period: "2023-Q4".to_string(),
            methodology: "综合评估法".to_string(),
            evidence_sources: vec![
                "生产数据分析".to_string(),
                "客户满意度调查".to_string(),
                "实地考察报告".to_string(),
            ],
            improvement_suggestions: vec![
                "加强质量控制体系".to_string(),
                "提升客户服务水平".to_string(),
            ],
            additional_comments: "总体表现良好，具有持续改进潜力".to_string(),
        }
    }

    /// 生成测试价格信息
    pub fn create_price_info(&self, base_price: u64) -> PriceInfo {
        PriceInfo {
            current_price: base_price,
            historical_average: base_price * 95 / 100, // 95% of current price
            volatility_score: 0.15,
            confidence_level: 95,
            data_source: "测试数据源".to_string(),
            last_updated: get_test_timestamp(),
        }
    }

    /// 生成测试消费证明
    pub fn create_consumption_proof(&self, quantity: u32, unit_price: u64) -> ConsumptionProof {
        ConsumptionProof {
            location: "北京市朝阳区".to_string(),
            timestamp: get_test_timestamp(),
            quantity,
            unit_price,
            total_amount: quantity as u64 * unit_price,
            payment_method: "数字货币".to_string(),
            receipt_hash: format!("hash_{}", self.counter),
            witness_signatures: vec![],
        }
    }
}

// ================================
// 测试环境工具
// ================================

pub struct TestEnvironment {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub mock_data: MockDataGenerator,
    pub test_mint: Keypair,
    pub platform_authority: Keypair,
}

impl TestEnvironment {
    /// 创建测试环境
    pub async fn new() -> Self {
        let program = ProgramTest::new("soonshop_core", PROGRAM_ID, processor!(soonshop_core::entry));
        let (banks_client, payer, recent_blockhash) = program.start().await;

        Self {
            banks_client,
            payer,
            recent_blockhash,
            mock_data: MockDataGenerator::new(),
            test_mint: Keypair::new(),
            platform_authority: Keypair::new(),
        }
    }

    /// 初始化代币铸造账户
    pub async fn setup_token_mint(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let rent = self.banks_client.get_rent().await?;
        let mint_rent = rent.minimum_balance(token::Mint::LEN);

        let create_mint_ix = system_instruction::create_account(
            &self.payer.pubkey(),
            &self.test_mint.pubkey(),
            mint_rent,
            token::Mint::LEN as u64,
            &token::ID,
        );

        let init_mint_ix = token::instruction::initialize_mint(
            &token::ID,
            &self.test_mint.pubkey(),
            &self.platform_authority.pubkey(),
            None,
            6, // 6 decimals for USDC-like token
        )?;

        let transaction = Transaction::new_signed_with_payer(
            &[create_mint_ix, init_mint_ix],
            Some(&self.payer.pubkey()),
            &[&self.payer, &self.test_mint],
            self.recent_blockhash,
        );

        self.banks_client.process_transaction(transaction).await?;
        Ok(())
    }

    /// 创建代币账户
    pub async fn create_token_account(&mut self, owner: &Pubkey) -> Result<Pubkey, Box<dyn std::error::Error>> {
        let token_account = Keypair::new();
        let rent = self.banks_client.get_rent().await?;
        let account_rent = rent.minimum_balance(token::TokenAccount::LEN);

        let create_account_ix = system_instruction::create_account(
            &self.payer.pubkey(),
            &token_account.pubkey(),
            account_rent,
            token::TokenAccount::LEN as u64,
            &token::ID,
        );

        let init_account_ix = token::instruction::initialize_account(
            &token::ID,
            &token_account.pubkey(),
            &self.test_mint.pubkey(),
            owner,
        )?;

        let transaction = Transaction::new_signed_with_payer(
            &[create_account_ix, init_account_ix],
            Some(&self.payer.pubkey()),
            &[&self.payer, &token_account],
            self.recent_blockhash,
        );

        self.banks_client.process_transaction(transaction).await?;
        Ok(token_account.pubkey())
    }

    /// 铸造代币到指定账户
    pub async fn mint_tokens(&mut self, to: &Pubkey, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mint_to_ix = token::instruction::mint_to(
            &token::ID,
            &self.test_mint.pubkey(),
            to,
            &self.platform_authority.pubkey(),
            &[],
            amount,
        )?;

        let transaction = Transaction::new_signed_with_payer(
            &[mint_to_ix],
            Some(&self.payer.pubkey()),
            &[&self.payer, &self.platform_authority],
            self.recent_blockhash,
        );

        self.banks_client.process_transaction(transaction).await?;
        Ok(())
    }

    /// 获取账户余额
    pub async fn get_token_balance(&mut self, account: &Pubkey) -> Result<u64, Box<dyn std::error::Error>> {
        let account_info = self.banks_client.get_account(*account).await?.unwrap();
        let token_account = token::TokenAccount::unpack(&account_info.data)?;
        Ok(token_account.amount)
    }
}

// ================================
// PDA辅助函数
// ================================

pub fn get_platform_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PLATFORM_CONFIG_SEED], &PROGRAM_ID)
}

pub fn get_user_account_pda(user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[USER_ACCOUNT_SEED, user.as_ref()], &PROGRAM_ID)
}

pub fn get_voucher_pda(voucher_id: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VOUCHER_SEED, voucher_id.as_bytes()], &PROGRAM_ID)
}

pub fn get_voucher_claim_pda(claim_id: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VOUCHER_CLAIM_SEED, claim_id.as_bytes()], &PROGRAM_ID)
}

pub fn get_consumption_pda(consumption_id: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONSUMPTION_SEED, consumption_id.as_bytes()], &PROGRAM_ID)
}

pub fn get_evaluation_pda(enterprise_id: &str, evaluator: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[EVALUATION_SEED, enterprise_id.as_bytes(), evaluator.as_ref()],
        &PROGRAM_ID,
    )
}

pub fn get_price_data_pda(product_category: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PRICE_DATA_SEED, product_category.as_bytes()], &PROGRAM_ID)
}

// ================================
// 时间工具函数
// ================================

/// 获取测试用的时间戳
pub fn get_test_timestamp() -> i64 {
    1640995200 // 2022-01-01 00:00:00 UTC
}

/// 获取未来时间戳
pub fn get_future_timestamp(days: i64) -> i64 {
    get_test_timestamp() + days * SECONDS_PER_DAY
}

/// 获取过去时间戳
pub fn get_past_timestamp(days: i64) -> i64 {
    get_test_timestamp() - days * SECONDS_PER_DAY
}

// ================================
// 断言辅助函数
// ================================

/// 断言两个公钥相等
pub fn assert_pubkey_eq(actual: &Pubkey, expected: &Pubkey, message: &str) {
    assert_eq!(actual, expected, "{}", message);
}

/// 断言数值在范围内
pub fn assert_in_range(value: u64, min: u64, max: u64, message: &str) {
    assert!(value >= min && value <= max, "{}: {} not in range [{}, {}]", message, value, min, max);
}

/// 断言百分比接近
pub fn assert_percentage_close(actual: f64, expected: f64, tolerance: f64, message: &str) {
    let diff = (actual - expected).abs();
    assert!(diff <= tolerance, "{}: {} not close to {} (tolerance: {})", message, actual, expected, tolerance);
}

/// 断言交易成功
pub async fn assert_transaction_success(
    banks_client: &mut BanksClient,
    transaction: Transaction,
    message: &str,
) {
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "{}: {:?}", message, result.err());
}

/// 断言交易失败
pub async fn assert_transaction_fails(
    banks_client: &mut BanksClient,
    transaction: Transaction,
    message: &str,
) {
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err(), "{}: transaction should have failed", message);
}

// ================================
// 数据验证工具
// ================================

/// 验证平台配置的有效性
pub fn validate_platform_config(config: &PlatformConfig) -> bool {
    config.base_multiplier >= MIN_MULTIPLIER
        && config.base_multiplier <= config.max_multiplier
        && config.max_multiplier <= MAX_MULTIPLIER
        && config.platform_fee_rate <= MAX_PLATFORM_FEE_RATE
        && config.created_at > 0
        && config.last_updated >= config.created_at
}

/// 验证用户账户的有效性
pub fn validate_user_account(account: &UserAccount) -> bool {
    !account.username.is_empty()
        && account.username.len() <= MAX_USERNAME_LENGTH
        && account.email.contains('@')
        && account.reputation_score <= 100
        && account.created_at > 0
}

/// 验证提货券的有效性
pub fn validate_voucher(voucher: &Voucher) -> bool {
    !voucher.id.is_empty()
        && voucher.total_quantity > 0
        && voucher.claimed_quantity <= voucher.total_quantity
        && voucher.consumed_quantity <= voucher.claimed_quantity
        && voucher.product_info.price > 0
        && voucher.created_at > 0
}

// ================================
// 随机数据生成器
// ================================

pub struct RandomDataGenerator {
    seed: u64,
}

impl RandomDataGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// 生成随机数
    pub fn next_u64(&mut self) -> u64 {
        // 简单的线性同余生成器
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }

    /// 生成指定范围内的随机数
    pub fn next_range(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next_u64() % (max - min + 1))
    }

    /// 生成随机价格
    pub fn next_price(&mut self) -> u64 {
        self.next_range(100000, 10000000) // 0.1 to 10.0 USDC
    }

    /// 生成随机数量
    pub fn next_quantity(&mut self) -> u32 {
        self.next_range(1, 1000) as u32
    }

    /// 生成随机评分
    pub fn next_score(&mut self) -> u8 {
        self.next_range(60, 100) as u8
    }
}

// ================================
// 性能测试工具
// ================================

use std::time::Instant;

pub struct PerformanceMeter {
    start_time: Instant,
    checkpoints: Vec<(String, Instant)>,
}

impl PerformanceMeter {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            checkpoints: Vec::new(),
        }
    }

    pub fn checkpoint(&mut self, name: &str) {
        self.checkpoints.push((name.to_string(), Instant::now()));
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    pub fn print_report(&self) {
        println!("=== 性能报告 ===");
        println!("总耗时: {:?}", self.elapsed());
        
        for (i, (name, time)) in self.checkpoints.iter().enumerate() {
            let duration = if i == 0 {
                time.duration_since(self.start_time)
            } else {
                time.duration_since(self.checkpoints[i - 1].1)
            };
            println!("{}: {:?}", name, duration);
        }
    }
}

// ================================
// 默认实现
// ================================

impl Default for MockDataGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RandomDataGenerator {
    fn default() -> Self {
        Self::new(42) // 固定种子，确保测试可重现
    }
}

impl Default for PerformanceMeter {
    fn default() -> Self {
        Self::new()
    }
} 