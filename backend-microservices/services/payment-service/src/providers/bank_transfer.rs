use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, instrument};
use uuid::Uuid;
use std::collections::HashMap;

use crate::config::BankTransferConfig;
use crate::error::PaymentError;

/// 银行转账支付提供商
#[derive(Clone)]
pub struct BankTransferProvider {
    config: BankTransferConfig,
}

/// 银行转账指令
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankTransferInstruction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub reference_number: String,
    pub recipient_account: BankAccount,
    pub sender_account: Option<BankAccount>,
    pub status: BankTransferStatus,
    pub description: Option<String>,
    pub expected_completion: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 银行账户信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankAccount {
    pub account_number: String,
    pub routing_number: String,
    pub account_name: String,
    pub bank_name: String,
    pub bank_address: Option<String>,
    pub swift_code: Option<String>,
    pub iban: Option<String>,
}

/// 银行转账状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BankTransferStatus {
    /// 待处理
    Pending,
    /// 已提交到银行
    Submitted,
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 已失败
    Failed,
    /// 已取消
    Cancelled,
    /// 需要验证
    RequiresVerification,
}

/// 银行转账创建请求
#[derive(Serialize, Debug)]
pub struct CreateBankTransferRequest {
    pub amount: f64,
    pub currency: String,
    pub recipient_account: BankAccount,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// 银行转账验证请求
#[derive(Serialize, Debug)]
pub struct VerifyBankTransferRequest {
    pub transfer_id: String,
    pub verification_code: Option<String>,
    pub verification_documents: Option<Vec<String>>,
}

/// 银行转账批次
#[derive(Serialize, Deserialize, Debug)]
pub struct BankTransferBatch {
    pub id: String,
    pub transfers: Vec<BankTransferInstruction>,
    pub total_amount: f64,
    pub currency: String,
    pub status: BankTransferStatus,
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl BankTransferProvider {
    /// 创建新的银行转账支付提供商实例
    pub async fn new(config: &BankTransferConfig) -> Result<Self> {
        info!("Initializing bank transfer provider");
        
        Ok(BankTransferProvider {
            config: config.clone(),
        })
    }

    /// 创建银行转账指令
    #[instrument(skip(self))]
    pub async fn create_transfer(
        &self,
        request: CreateBankTransferRequest,
    ) -> Result<BankTransferInstruction> {
        let transfer_id = Uuid::new_v4().to_string();
        let reference_number = self.generate_reference_number();
        
        info!("Creating bank transfer: {} for amount: {} {}", 
              transfer_id, request.amount, request.currency);

        // 验证金额限制
        if request.amount > self.config.max_transfer_amount {
            return Err(PaymentError::InvalidAmount(
                format!("Amount {} exceeds maximum limit of {}", 
                       request.amount, self.config.max_transfer_amount)
            ).into());
        }

        if request.amount < self.config.min_transfer_amount {
            return Err(PaymentError::InvalidAmount(
                format!("Amount {} is below minimum limit of {}", 
                       request.amount, self.config.min_transfer_amount)
            ).into());
        }

        // 验证收款账户
        self.validate_bank_account(&request.recipient_account)?;

        let now = chrono::Utc::now();
        let expected_completion = now + chrono::Duration::days(self.config.processing_days as i64);

        let transfer = BankTransferInstruction {
            id: transfer_id,
            amount: request.amount,
            currency: request.currency.to_uppercase(),
            reference_number,
            recipient_account: request.recipient_account,
            sender_account: None, // 将在处理时设置
            status: BankTransferStatus::Pending,
            description: request.description,
            expected_completion: Some(expected_completion),
            metadata: request.metadata.unwrap_or_default(),
            created_at: now,
            updated_at: now,
        };

        info!("Successfully created bank transfer instruction: {}", transfer.id);
        Ok(transfer)
    }

    /// 验证银行转账
    #[instrument(skip(self))]
    pub async fn verify_transfer(
        &self,
        request: VerifyBankTransferRequest,
    ) -> Result<BankTransferInstruction> {
        info!("Verifying bank transfer: {}", request.transfer_id);

        // 这里应该实现实际的验证逻辑
        // 包括检查验证码、文档等

        // 模拟验证过程
        let mut transfer = self.get_transfer(&request.transfer_id).await?;
        
        if transfer.status != BankTransferStatus::RequiresVerification {
            return Err(PaymentError::InvalidRequest(
                "Transfer is not in verification status".to_string()
            ).into());
        }

        transfer.status = BankTransferStatus::Processing;
        transfer.updated_at = chrono::Utc::now();

        info!("Successfully verified bank transfer: {}", transfer.id);
        Ok(transfer)
    }

    /// 获取转账详情
    #[instrument(skip(self))]
    pub async fn get_transfer(&self, transfer_id: &str) -> Result<BankTransferInstruction> {
        // 这里应该从数据库或外部系统获取转账详情
        // 暂时返回模拟数据
        
        info!("Retrieving bank transfer: {}", transfer_id);
        
        // 模拟数据 - 实际应用中应该从存储中获取
        Err(PaymentError::NotFound(format!("Transfer {} not found", transfer_id)).into())
    }

    /// 取消转账
    #[instrument(skip(self))]
    pub async fn cancel_transfer(&self, transfer_id: &str) -> Result<BankTransferInstruction> {
        info!("Cancelling bank transfer: {}", transfer_id);

        let mut transfer = self.get_transfer(transfer_id).await?;
        
        // 只有特定状态的转账可以取消
        match transfer.status {
            BankTransferStatus::Pending | 
            BankTransferStatus::RequiresVerification => {
                transfer.status = BankTransferStatus::Cancelled;
                transfer.updated_at = chrono::Utc::now();
                
                info!("Successfully cancelled bank transfer: {}", transfer.id);
                Ok(transfer)
            }
            _ => {
                Err(PaymentError::InvalidRequest(
                    format!("Cannot cancel transfer in status: {:?}", transfer.status)
                ).into())
            }
        }
    }

    /// 创建批量转账
    #[instrument(skip(self))]
    pub async fn create_batch_transfer(
        &self,
        transfers: Vec<CreateBankTransferRequest>,
    ) -> Result<BankTransferBatch> {
        let batch_id = Uuid::new_v4().to_string();
        
        info!("Creating batch transfer: {} with {} transfers", batch_id, transfers.len());

        if transfers.len() > self.config.max_batch_size {
            return Err(PaymentError::InvalidRequest(
                format!("Batch size {} exceeds maximum of {}", 
                       transfers.len(), self.config.max_batch_size)
            ).into());
        }

        let mut batch_transfers = Vec::new();
        let mut total_amount = 0.0;
        let currency = transfers.first()
            .ok_or(PaymentError::InvalidRequest("Empty batch".to_string()))?
            .currency
            .clone();

        for request in transfers {
            // 验证所有转账使用相同货币
            if request.currency != currency {
                return Err(PaymentError::InvalidRequest(
                    "All transfers in batch must use same currency".to_string()
                ).into());
            }

            let transfer = self.create_transfer(request).await?;
            total_amount += transfer.amount;
            batch_transfers.push(transfer);
        }

        let batch = BankTransferBatch {
            id: batch_id,
            transfers: batch_transfers,
            total_amount,
            currency,
            status: BankTransferStatus::Pending,
            submitted_at: None,
            processed_at: None,
            created_at: chrono::Utc::now(),
        };

        info!("Successfully created batch transfer: {} with total amount: {} {}", 
              batch.id, batch.total_amount, batch.currency);
        Ok(batch)
    }

    /// 提交批量转账
    #[instrument(skip(self))]
    pub async fn submit_batch(&self, batch_id: &str) -> Result<BankTransferBatch> {
        info!("Submitting batch transfer: {}", batch_id);

        // 这里应该实现实际的银行API调用
        // 暂时模拟提交过程

        // 模拟数据 - 实际应用中应该从存储中获取
        Err(PaymentError::NotFound(format!("Batch {} not found", batch_id)).into())
    }

    /// 生成参考号
    fn generate_reference_number(&self) -> String {
        let prefix = &self.config.reference_prefix;
        let timestamp = chrono::Utc::now().timestamp();
        let random = rand::random::<u32>();
        
        format!("{}{}{:08X}", prefix, timestamp, random)
    }

    /// 验证银行账户
    fn validate_bank_account(&self, account: &BankAccount) -> Result<()> {
        if account.account_number.is_empty() {
            return Err(PaymentError::InvalidRequest("Account number is required".to_string()).into());
        }

        if account.routing_number.is_empty() {
            return Err(PaymentError::InvalidRequest("Routing number is required".to_string()).into());
        }

        if account.account_name.is_empty() {
            return Err(PaymentError::InvalidRequest("Account name is required".to_string()).into());
        }

        if account.bank_name.is_empty() {
            return Err(PaymentError::InvalidRequest("Bank name is required".to_string()).into());
        }

        // 验证账户号码格式（简单验证）
        if account.account_number.len() < 8 || account.account_number.len() > 20 {
            return Err(PaymentError::InvalidRequest("Invalid account number format".to_string()).into());
        }

        // 验证路由号码格式（美国银行路由号码为9位）
        if account.routing_number.len() != 9 {
            warn!("Routing number length is not 9 digits: {}", account.routing_number.len());
        }

        Ok(())
    }

    /// 健康检查
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> Result<()> {
        info!("Bank transfer provider health check passed");
        // 银行转账提供商通常没有实时API可以检查
        // 这里可以检查配置是否正确
        
        if self.config.processing_days == 0 {
            return Err(PaymentError::ConfigurationError(
                "Processing days must be greater than 0".to_string()
            ).into());
        }

        Ok(())
    }

    /// 获取支持的货币
    pub fn get_supported_currencies(&self) -> &Vec<String> {
        &self.config.supported_currencies
    }

    /// 检查是否支持指定货币
    pub fn is_currency_supported(&self, currency: &str) -> bool {
        self.config.supported_currencies
            .iter()
            .any(|c| c.eq_ignore_ascii_case(currency))
    }

    /// 获取转账费用
    pub fn calculate_transfer_fee(&self, amount: f64, currency: &str) -> f64 {
        if !self.is_currency_supported(currency) {
            return 0.0; // 不支持的货币不收费
        }

        // 计算固定费用 + 比例费用
        let percentage_fee = amount * self.config.fee_percentage / 100.0;
        let total_fee = self.config.fixed_fee + percentage_fee;
        
        // 确保费用不超过最大限制
        total_fee.min(self.config.max_fee)
    }
} 