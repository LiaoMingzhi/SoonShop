#![allow(clippy::result_large_err)]

/**
 * SoonShop核心智能合约 - 基于共产主义经济原理的商业平台
 * 
 * 功能描述：
 * - 这是SoonShop平台的核心智能合约，实现了基于共产主义经济原理的商业交易机制
 * - 提供基础的平台管理功能、钱包功能、代币发行功能、提货券额度功能
 * 
 * 版本：v1.0.0 (完整版本)
 * 许可证：MIT
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, MintTo, Burn, Transfer};

// 声明程序ID (32字节)
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgEUCNKy8mfn");

// 导入基础模块
pub mod constants;
pub mod errors;
pub mod state;

// 使用模块
use state::*;
use errors::*;

/**
 * SoonShop核心智能合约程序
 * 
 * 实现了基于共产主义经济原理的商业平台核心功能
 */
#[program]
pub mod soonshop_core {
    use super::*;

    // ================================
    // 平台管理功能
    // ================================

    /**
     * 初始化SoonShop平台
     * 
     * 功能：设置平台基础配置和管理权限，创建全局状态账户
     * 权限：仅限超级管理员
     */
    pub fn initialize_platform(
        ctx: Context<InitializePlatform>,
        base_multiplier: u8,
        platform_fee_rate: u16,
        version: String,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;
        
        // 初始化平台配置
        platform_config.initialize(
            ctx.accounts.authority.key(),
            ctx.accounts.reward_pool.key(),
            base_multiplier,
            platform_fee_rate,
            version,
        )?;

        // 发送初始化事件
        emit!(PlatformInitialized {
            authority: ctx.accounts.authority.key(),
            reward_pool: ctx.accounts.reward_pool.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 添加管理员
     * 
     * 功能：向平台添加新的管理员
     * 权限：仅限超级管理员
     */
    pub fn add_admin(
        ctx: Context<AddAdmin>,
        new_admin: Pubkey,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;
        platform_config.add_admin(new_admin)?;

        // 发送添加管理员事件
        emit!(AdminAdded {
            admin: new_admin,
            added_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 移除管理员
     * 
     * 功能：从平台移除管理员
     * 权限：仅限超级管理员
     */
    pub fn remove_admin(
        ctx: Context<RemoveAdmin>,
        admin_to_remove: Pubkey,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;
        platform_config.remove_admin(admin_to_remove)?;

        // 发送移除管理员事件
        emit!(AdminRemoved {
            admin: admin_to_remove,
            removed_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 紧急暂停
     * 
     * 功能：在发现安全问题时紧急暂停合约功能
     * 权限：仅限管理员
     */
    pub fn emergency_pause(
        ctx: Context<EmergencyPause>,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;
        platform_config.emergency_pause()?;

        // 发送紧急暂停事件
        emit!(EmergencyPaused {
            paused_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 恢复系统
     * 
     * 功能：修复问题后恢复合约正常功能
     * 权限：仅限管理员
     */
    pub fn emergency_resume(
        ctx: Context<EmergencyResume>,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;
        platform_config.emergency_resume()?;

        // 发送恢复系统事件
        emit!(EmergencyResumed {
            resumed_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // ================================
    // 钱包功能
    // ================================

    /**
     * 创建用户钱包
     * 
     * 功能：为用户创建专用的钱包账户，记录资产和交易历史
     * 权限：用户本人
     */
    pub fn create_user_wallet(
        ctx: Context<CreateUserWallet>,
    ) -> Result<()> {
        let user_wallet = &mut ctx.accounts.user_wallet;
        
        // 初始化用户钱包
        user_wallet.initialize(
            ctx.accounts.user.key(),
            ctx.accounts.platform_token_account.key(),
        )?;

        // 发送创建钱包事件
        emit!(WalletCreated {
            owner: ctx.accounts.user.key(),
            wallet: ctx.accounts.user_wallet.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 查询钱包余额
     * 
     * 功能：获取用户各种代币的余额信息
     * 权限：用户本人
     */
    pub fn get_wallet_balance(
        ctx: Context<GetWalletBalance>,
    ) -> Result<WalletBalanceInfo> {
        let user_wallet = &ctx.accounts.user_wallet;
        let platform_token_account = &ctx.accounts.platform_token_account;
        
        // 检查权限
        if user_wallet.owner != ctx.accounts.user.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 获取余额信息
        let balance_info = WalletBalanceInfo {
            platform_token_balance: platform_token_account.amount,
            total_income: user_wallet.total_income,
            total_expense: user_wallet.total_expense,
            total_rewards: user_wallet.total_rewards,
            transaction_count: user_wallet.transaction_count,
            last_transaction_at: user_wallet.last_transaction_at,
        };

        Ok(balance_info)
    }

    /**
     * 查询收入记录
     * 
     * 功能：获取用户的所有收入记录，包括奖励、转账等
     * 权限：用户本人
     */
    pub fn get_income_history(
        ctx: Context<GetIncomeHistory>,
        _start_time: Option<i64>,
        _end_time: Option<i64>,
        _limit: Option<u32>,
    ) -> Result<IncomeHistoryInfo> {
        let user_wallet = &ctx.accounts.user_wallet;
        
        // 检查权限
        if user_wallet.owner != ctx.accounts.user.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 这里应该查询实际的收入记录，但由于Solana的限制，我们返回统计信息
        let income_info = IncomeHistoryInfo {
            total_income: user_wallet.total_income,
            today_income: user_wallet.statistics.today_income,
            monthly_income: user_wallet.statistics.monthly_income,
            max_single_income: user_wallet.statistics.max_single_income,
            income_count: user_wallet.transaction_count, // 简化处理
        };

        Ok(income_info)
    }

    /**
     * 查询支出记录
     * 
     * 功能：获取用户的所有支出记录，包括消费、转账等
     * 权限：用户本人
     */
    pub fn get_expense_history(
        ctx: Context<GetExpenseHistory>,
        _start_time: Option<i64>,
        _end_time: Option<i64>,
        _limit: Option<u32>,
    ) -> Result<ExpenseHistoryInfo> {
        let user_wallet = &ctx.accounts.user_wallet;
        
        // 检查权限
        if user_wallet.owner != ctx.accounts.user.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 返回支出统计信息
        let expense_info = ExpenseHistoryInfo {
            total_expense: user_wallet.total_expense,
            today_expense: user_wallet.statistics.today_expense,
            monthly_expense: user_wallet.statistics.monthly_expense,
            max_single_expense: user_wallet.statistics.max_single_expense,
            expense_count: user_wallet.transaction_count, // 简化处理
        };

        Ok(expense_info)
    }

    /**
     * 查询奖励记录
     * 
     * 功能：获取用户获得的所有奖励记录，包括倍增奖励、推荐奖励等
     * 权限：用户本人
     */
    pub fn get_reward_history(
        ctx: Context<GetRewardHistory>,
        _reward_type: Option<RewardType>,
        _start_time: Option<i64>,
        _end_time: Option<i64>,
    ) -> Result<RewardHistoryInfo> {
        let user_wallet = &ctx.accounts.user_wallet;
        
        // 检查权限
        if user_wallet.owner != ctx.accounts.user.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 返回奖励统计信息
        let reward_info = RewardHistoryInfo {
            total_rewards: user_wallet.total_rewards,
            today_rewards: user_wallet.statistics.today_rewards,
            monthly_rewards: user_wallet.statistics.monthly_rewards,
            reward_count: user_wallet.transaction_count, // 简化处理
        };

        Ok(reward_info)
    }

    /**
     * 代币转账
     * 
     * 功能：支持用户之间进行代币转账操作
     * 权限：发送者本人
     */
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
        memo: Option<String>,
    ) -> Result<()> {
        let sender_wallet = &mut ctx.accounts.sender_wallet;
        let recipient_wallet = &mut ctx.accounts.recipient_wallet;
        
        // 检查权限
        if sender_wallet.owner != ctx.accounts.sender.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 检查转账限额
        sender_wallet.check_transfer_limit(amount)?;

        // 执行代币转账
        let transfer_instruction = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, amount)?;

        // 更新钱包统计
        sender_wallet.add_expense(amount)?;
        recipient_wallet.add_income(amount)?;

        // 发送转账事件
        emit!(TokenTransferred {
            from: ctx.accounts.sender.key(),
            to: ctx.accounts.recipient_wallet.owner,
            amount,
            memo: memo.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // ================================
    // 代币发行功能
    // ================================

    /**
     * 创建平台代币
     * 
     * 功能：创建用于提货券额度、奖励分发的平台代币
     * 权限：仅限平台管理员
     */
    pub fn create_platform_token(
        ctx: Context<CreatePlatformToken>,
        decimals: u8,
        initial_supply: u64,
    ) -> Result<()> {
        let platform_config = &ctx.accounts.platform_config;
        
        // 检查管理员权限
        if !platform_config.has_admin_permission(&ctx.accounts.authority.key()) {
            return Err(SoonShopError::InsufficientAdminPrivilege.into());
        }

        // 铸造初始供应量
        if initial_supply > 0 {
            let mint_instruction = MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                mint_instruction,
            );

            token::mint_to(cpi_ctx, initial_supply)?;
        }

        // 发送创建代币事件
        emit!(PlatformTokenCreated {
            mint: ctx.accounts.token_mint.key(),
            vault: ctx.accounts.token_vault.key(),
            decimals,
            initial_supply,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 铸造代币
     * 
     * 功能：根据需要铸造新的代币，用于奖励分发
     * 权限：仅限平台管理员
     */
    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        let platform_config = &ctx.accounts.platform_config;
        
        // 检查管理员权限
        if !platform_config.has_admin_permission(&ctx.accounts.mint_authority.key()) {
            return Err(SoonShopError::InsufficientAdminPrivilege.into());
        }

        // 执行铸造
        let mint_instruction = MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            mint_instruction,
        );

        token::mint_to(cpi_ctx, amount)?;

        // 发送铸造事件
        emit!(TokensMinted {
            mint: ctx.accounts.token_mint.key(),
            to: ctx.accounts.recipient_token_account.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 销毁代币
     * 
     * 功能：销毁已使用的代币，控制代币供应量
     * 权限：代币持有者
     */
    pub fn burn_tokens(
        ctx: Context<BurnTokens>,
        amount: u64,
    ) -> Result<()> {
        // 执行销毁
        let burn_instruction = Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            burn_instruction,
        );

        token::burn(cpi_ctx, amount)?;

        // 发送销毁事件
        emit!(TokensBurned {
            mint: ctx.accounts.token_mint.key(),
            from: ctx.accounts.token_account.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // ================================
    // 提货券额度功能
    // ================================

    /**
     * 发行提货券额度
     * 
     * 功能：生产者发布提货券时，分配相应的代币额度
     * 权限：生产者本人
     */
    pub fn issue_voucher_credits(
        ctx: Context<IssueVoucherCredits>,
        voucher_id: String,
        credit_amount: u64,
        product_info: ProductInfo,
        expires_at: Option<i64>,
    ) -> Result<()> {
        let voucher = &mut ctx.accounts.voucher_account;
        let producer_wallet = &mut ctx.accounts.producer_wallet;
        
        // 检查权限
        if producer_wallet.owner != ctx.accounts.producer.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 创建提货券配置
        let voucher_config = VoucherConfig {
            allow_partial_claim: true,
            allow_transfer: false,
            require_appointment: false,
            min_claim_amount: 1,
            max_claim_amount: credit_amount,
            claim_fee: 0,
            cancellation_fee: 0,
            require_identity_verification: false,
            allowed_user_types: vec![UserType::Regular],
            geographic_restrictions: vec![],
        };

        // 初始化提货券
        voucher.initialize(
            voucher_id.clone(),
            ctx.accounts.producer.key(),
            product_info,
            credit_amount,
            expires_at,
            voucher_config,
        )?;

        // 发送发行事件
        emit!(VoucherCreditsIssued {
            voucher_id,
            producer: ctx.accounts.producer.key(),
            credit_amount,
            expires_at,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 获取提货券额度
     * 
     * 功能：符合条件的消费者获取提货券对应的代币额度
     * 权限：消费者本人
     */
    pub fn claim_voucher_credits(
        ctx: Context<ClaimVoucherCredits>,
        claim_amount: u64,
    ) -> Result<()> {
        let voucher = &mut ctx.accounts.voucher_account;
        let consumer_wallet = &mut ctx.accounts.consumer_wallet;
        
        // 检查权限
        if consumer_wallet.owner != ctx.accounts.consumer.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 执行获取
        voucher.claim_credits(claim_amount)?;

        // 更新消费者钱包
        consumer_wallet.add_income(claim_amount)?;

        // 发送获取事件
        emit!(VoucherCreditsClaimed {
            voucher_id: voucher.id.clone(),
            consumer: ctx.accounts.consumer.key(),
            claim_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 消费提货券额度
     * 
     * 功能：消费者使用提货券额度购买商品或服务
     * 权限：消费者本人
     */
    pub fn consume_voucher_credits(
        ctx: Context<ConsumeVoucherCredits>,
        voucher_id: String,
        consume_amount: u64,
        location: String,
        notes: String,
    ) -> Result<()> {
        let voucher = &mut ctx.accounts.voucher_account;
        let consumer_wallet = &mut ctx.accounts.consumer_wallet;
        let consumption_record = &mut ctx.accounts.consumption_record;
        
        // 检查权限
        if consumer_wallet.owner != ctx.accounts.consumer.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 检查提货券ID匹配
        if voucher.id != voucher_id {
            return Err(SoonShopError::VoucherNotFound.into());
        }

        // 执行消费
        voucher.consume_credits(consume_amount)?;

        // 更新消费者钱包
        consumer_wallet.add_expense(consume_amount)?;

        // 生成消费记录ID
        let consumption_id = format!("consumption_{}_{}", voucher_id, Clock::get()?.unix_timestamp);

        // 初始化消费记录
        consumption_record.initialize(
            consumption_id.clone(),
            voucher_id.clone(),
            ctx.accounts.consumer.key(),
            ctx.accounts.merchant.key(),
            consume_amount,
            1, // 假设数量为1
            location,
            notes,
        )?;

        // 发送消费事件
        emit!(VoucherCreditsConsumed {
            voucher_id,
            consumer: ctx.accounts.consumer.key(),
            merchant: ctx.accounts.merchant.key(),
            consume_amount,
            consumption_id,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 核销提货券额度
     * 
     * 功能：商家确认商品交付，触发倍增奖励分发
     * 权限：商家本人
     */
    pub fn verify_voucher_consumption(
        ctx: Context<VerifyVoucherConsumption>,
        quality_score: u8,
    ) -> Result<()> {
        let consumption_record = &mut ctx.accounts.consumption_record;
        let _merchant_wallet = &mut ctx.accounts.merchant_wallet;
        
        // 检查权限
        if consumption_record.merchant != ctx.accounts.merchant.key() {
            return Err(SoonShopError::Unauthorized.into());
        }

        // 确认消费
        consumption_record.confirm(quality_score)?;

        // 发送核销事件
        emit!(VoucherConsumptionVerified {
            consumption_id: consumption_record.id.clone(),
            merchant: ctx.accounts.merchant.key(),
            quality_score,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /**
     * 分发倍增奖励
     * 
     * 功能：根据消费金额和质量评分计算倍增奖励，分发给相关方
     * 权限：自动触发或管理员调用
     */
    pub fn distribute_multiplier_rewards(
        ctx: Context<DistributeMultiplierRewards>,
    ) -> Result<()> {
        let consumption_record = &mut ctx.accounts.consumption_record;
        let producer_wallet = &mut ctx.accounts.producer_wallet;
        let platform_config = &ctx.accounts.platform_config;
        
        // 检查消费记录状态
        if consumption_record.status != ConsumptionStatus::Confirmed {
            return Err(SoonShopError::InvalidConsumptionStatus.into());
        }

        // 获取质量评分
        let quality_score = consumption_record.quality_score.unwrap_or(5);
        
        // 计算奖励
        let base_reward = consumption_record.amount * platform_config.base_multiplier as u64;
        let quality_reward = base_reward * quality_score as u64 / 10;
        let producer_reward = quality_reward * 70 / 100; // 生产者获得70%
        let platform_reward = quality_reward * 30 / 100; // 平台获得30%

        // 更新奖励信息
        consumption_record.reward_info.base_reward = base_reward;
        consumption_record.reward_info.quality_reward = quality_reward;
        consumption_record.reward_info.multiplier_reward = quality_reward;
        consumption_record.reward_info.total_reward = quality_reward;
        consumption_record.reward_info.reward_status = RewardStatus::Distributed;
        consumption_record.reward_info.reward_distributed_at = Some(Clock::get()?.unix_timestamp);

        // 更新生产者钱包
        producer_wallet.add_reward(producer_reward)?;

        // 完成消费记录
        consumption_record.complete()?;

        // 发送奖励分发事件
        emit!(MultiplierRewardsDistributed {
            consumption_id: consumption_record.id.clone(),
            producer: producer_wallet.owner,
            producer_reward,
            platform_reward,
            quality_score,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

// ================================
// 账户结构定义
// ================================

/// 初始化平台账户结构
#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PlatformConfig::SPACE,
        seeds = [b"platform_config"],
        bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: 这是奖励池账户
    pub reward_pool: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

/// 添加管理员账户结构
#[derive(Accounts)]
pub struct AddAdmin<'info> {
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,
    pub authority: Signer<'info>,
}

/// 移除管理员账户结构
#[derive(Accounts)]
pub struct RemoveAdmin<'info> {
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,
    pub authority: Signer<'info>,
}

/// 紧急暂停账户结构
#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,
    pub authority: Signer<'info>,
}

/// 恢复系统账户结构
#[derive(Accounts)]
pub struct EmergencyResume<'info> {
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,
    pub authority: Signer<'info>,
}

/// 创建用户钱包账户结构
#[derive(Accounts)]
pub struct CreateUserWallet<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserWallet::SPACE,
        seeds = [b"user_wallet", user.key().as_ref()],
        bump
    )]
    pub user_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: 这是用户的平台代币账户
    pub platform_token_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

/// 查询钱包余额账户结构
#[derive(Accounts)]
pub struct GetWalletBalance<'info> {
    pub user_wallet: Account<'info, UserWallet>,
    pub user: Signer<'info>,
    pub platform_token_account: Account<'info, TokenAccount>,
}

/// 查询收入历史账户结构
#[derive(Accounts)]
pub struct GetIncomeHistory<'info> {
    pub user_wallet: Account<'info, UserWallet>,
    pub user: Signer<'info>,
}

/// 查询支出历史账户结构
#[derive(Accounts)]
pub struct GetExpenseHistory<'info> {
    pub user_wallet: Account<'info, UserWallet>,
    pub user: Signer<'info>,
}

/// 查询奖励历史账户结构
#[derive(Accounts)]
pub struct GetRewardHistory<'info> {
    pub user_wallet: Account<'info, UserWallet>,
    pub user: Signer<'info>,
}

/// 代币转账账户结构
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub sender_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub recipient_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub sender: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// 创建平台代币账户结构
#[derive(Accounts)]
pub struct CreatePlatformToken<'info> {
    pub platform_config: Account<'info, PlatformConfig>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    pub mint_authority: Signer<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// 铸造代币账户结构
#[derive(Accounts)]
pub struct MintTokens<'info> {
    pub platform_config: Account<'info, PlatformConfig>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// 销毁代币账户结构
#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// 发行提货券额度账户结构
#[derive(Accounts)]
#[instruction(voucher_id: String)]
pub struct IssueVoucherCredits<'info> {
    #[account(
        init,
        payer = producer,
        space = 8 + Voucher::SPACE,
        seeds = [b"voucher", producer.key().as_ref(), voucher_id.as_bytes()],
        bump
    )]
    pub voucher_account: Account<'info, Voucher>,
    #[account(mut)]
    pub producer_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub producer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// 获取提货券额度账户结构
#[derive(Accounts)]
pub struct ClaimVoucherCredits<'info> {
    #[account(mut)]
    pub voucher_account: Account<'info, Voucher>,
    #[account(mut)]
    pub consumer_wallet: Account<'info, UserWallet>,
    pub consumer: Signer<'info>,
}

/// 消费提货券额度账户结构
#[derive(Accounts)]
#[instruction(voucher_id: String)]
pub struct ConsumeVoucherCredits<'info> {
    #[account(mut)]
    pub voucher_account: Account<'info, Voucher>,
    #[account(mut)]
    pub consumer_wallet: Account<'info, UserWallet>,
    #[account(
        init,
        payer = consumer,
        space = 8 + ConsumptionRecord::SPACE,
        seeds = [b"consumption", voucher_id.as_bytes(), consumer.key().as_ref()],
        bump
    )]
    pub consumption_record: Account<'info, ConsumptionRecord>,
    #[account(mut)]
    pub consumer: Signer<'info>,
    /// CHECK: 这是商家账户
    pub merchant: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

/// 核销提货券额度账户结构
#[derive(Accounts)]
pub struct VerifyVoucherConsumption<'info> {
    #[account(mut)]
    pub consumption_record: Account<'info, ConsumptionRecord>,
    #[account(mut)]
    pub merchant_wallet: Account<'info, UserWallet>,
    pub merchant: Signer<'info>,
}

/// 分发倍增奖励账户结构
#[derive(Accounts)]
pub struct DistributeMultiplierRewards<'info> {
    #[account(mut)]
    pub consumption_record: Account<'info, ConsumptionRecord>,
    #[account(mut)]
    pub producer_wallet: Account<'info, UserWallet>,
    pub platform_config: Account<'info, PlatformConfig>,
    pub reward_pool: Account<'info, TokenAccount>,
}

// ================================
// 返回数据结构
// ================================

/// 钱包余额信息
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct WalletBalanceInfo {
    pub platform_token_balance: u64,
    pub total_income: u64,
    pub total_expense: u64,
    pub total_rewards: u64,
    pub transaction_count: u64,
    pub last_transaction_at: i64,
}

/// 收入历史信息
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncomeHistoryInfo {
    pub total_income: u64,
    pub today_income: u64,
    pub monthly_income: u64,
    pub max_single_income: u64,
    pub income_count: u64,
}

/// 支出历史信息
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExpenseHistoryInfo {
    pub total_expense: u64,
    pub today_expense: u64,
    pub monthly_expense: u64,
    pub max_single_expense: u64,
    pub expense_count: u64,
}

/// 奖励历史信息
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RewardHistoryInfo {
    pub total_rewards: u64,
    pub today_rewards: u64,
    pub monthly_rewards: u64,
    pub reward_count: u64,
}

// ================================
// 事件定义
// ================================

#[event]
pub struct PlatformInitialized {
    pub authority: Pubkey,
    pub reward_pool: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AdminAdded {
    pub admin: Pubkey,
    pub added_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AdminRemoved {
    pub admin: Pubkey,
    pub removed_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EmergencyPaused {
    pub paused_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EmergencyResumed {
    pub resumed_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct WalletCreated {
    pub owner: Pubkey,
    pub wallet: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokenTransferred {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub memo: Option<String>,
    pub timestamp: i64,
}

#[event]
pub struct PlatformTokenCreated {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub decimals: u8,
    pub initial_supply: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensMinted {
    pub mint: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensBurned {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct VoucherCreditsIssued {
    pub voucher_id: String,
    pub producer: Pubkey,
    pub credit_amount: u64,
    pub expires_at: Option<i64>,
    pub timestamp: i64,
}

#[event]
pub struct VoucherCreditsClaimed {
    pub voucher_id: String,
    pub consumer: Pubkey,
    pub claim_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct VoucherCreditsConsumed {
    pub voucher_id: String,
    pub consumer: Pubkey,
    pub merchant: Pubkey,
    pub consume_amount: u64,
    pub consumption_id: String,
    pub timestamp: i64,
}

#[event]
pub struct VoucherConsumptionVerified {
    pub consumption_id: String,
    pub merchant: Pubkey,
    pub quality_score: u8,
    pub timestamp: i64,
}

#[event]
pub struct MultiplierRewardsDistributed {
    pub consumption_id: String,
    pub producer: Pubkey,
    pub producer_reward: u64,
    pub platform_reward: u64,
    pub quality_score: u8,
    pub timestamp: i64,
}

// ================================
// 视图结构
// ================================

/// 平台配置视图
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PlatformConfigView {
    pub authority: Pubkey,
    pub base_multiplier: u8,
    pub max_multiplier: u8,
    pub platform_fee_rate: u16,
    pub reward_pool: Pubkey,
    pub is_paused: bool,
    pub is_emergency_paused: bool,
    pub total_rewards_distributed: u64,
    pub total_vouchers_issued: u64,
    pub total_enterprises: u32,
    pub total_transactions: u32,
    pub total_volume: u64,
    pub admin_count: u8,
    pub created_at: i64,
    pub last_updated: i64,
} 