use anyhow::Result;
use uuid::Uuid;
use crate::models::payment::{
    Payment, PaymentRefund, CreatePaymentRequest, ProcessPaymentRequest, 
    RefundPaymentRequest, PaymentStatus, PaymentMethod, Currency
};
use crate::providers::PaymentProviders;
use crate::db::Database;

#[derive(Clone)]
pub struct PaymentService {
    db: Database,
    providers: PaymentProviders,
}

impl PaymentService {
    pub async fn new(db: Database, config: crate::config::AppConfig) -> Result<Self> {
        let providers = PaymentProviders::new(&config).await?;
        Ok(Self { db, providers })
    }
    
    pub async fn create_payment(
        &self,
        req: &CreatePaymentRequest,
        user_id: Uuid,
    ) -> Result<Payment> {
        // 验证订单存在
        self.validate_order(&req.order_id, user_id).await?;
        
        // 计算费用
        let fee_amount = self.calculate_fee(&req.payment_method, &req.amount);
        let net_amount = req.amount - fee_amount;
        
        // 创建支付记录
        let payment = Payment {
            id: Uuid::new_v4(),
            order_id: req.order_id,
            user_id,
            payment_method: req.payment_method.clone(),
            status: PaymentStatus::Pending,
            amount: req.amount,
            currency: req.currency.clone(),
            fee_amount,
            net_amount,
            provider_payment_id: None,
            provider_name: None,
            transaction_hash: None,
            failure_reason: None,
            metadata: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            completed_at: None,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::minutes(15)),
        };
        
        // 保存到数据库
        self.db.create_payment(&payment).await?;
        
        // 根据支付方式初始化支付
        match req.payment_method {
            PaymentMethod::Solana => {
                // Solana支付不需要额外初始化
            }
            PaymentMethod::CreditCard => {
                // 初始化信用卡支付
            }
            PaymentMethod::PayPal => {
                // 初始化PayPal支付
            }
            _ => {}
        }
        
        Ok(payment)
    }
    
    pub async fn process_payment(
        &self,
        payment_id: Uuid,
        req: &ProcessPaymentRequest,
        user_id: Uuid,
    ) -> Result<Payment> {
        let mut payment = self.get_payment_by_id(payment_id, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("Payment not found"))?;
        
        // 检查支付状态
        if payment.status != PaymentStatus::Pending {
            return Err(anyhow::anyhow!("Payment cannot be processed"));
        }
        
        // 更新状态为处理中
        payment.status = PaymentStatus::Processing;
        payment.updated_at = chrono::Utc::now();
        self.db.update_payment(&payment).await?;
        
        // 根据支付方式处理
        match payment.payment_method {
            PaymentMethod::Solana => {
                self.process_solana_payment(&mut payment, req).await?;
            }
            PaymentMethod::CreditCard => {
                self.process_credit_card_payment(&mut payment, req).await?;
            }
            PaymentMethod::PayPal => {
                self.process_paypal_payment(&mut payment, req).await?;
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported payment method"));
            }
        }
        
        Ok(payment)
    }
    
    pub async fn get_payment_by_id(
        &self,
        payment_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Payment>> {
        self.db.get_payment_by_id(payment_id, user_id).await
    }
    
    pub async fn get_payment_status(
        &self,
        payment_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<PaymentStatus>> {
        if let Some(payment) = self.get_payment_by_id(payment_id, user_id).await? {
            Ok(Some(payment.status))
        } else {
            Ok(None)
        }
    }
    
    /// 取消支付
    pub async fn cancel_payment(
        &self, 
        payment_id: Uuid, 
        user_id: Uuid
    ) -> Result<Payment> {
        // TODO: 实现取消支付逻辑
        // 1. 验证支付存在且属于用户
        // 2. 检查支付状态是否可以取消
        // 3. 调用支付提供商取消支付
        // 4. 更新数据库状态
        
        tracing::info!("Cancelling payment {} for user {}", payment_id, user_id);
        
        // 临时返回默认支付对象
        Ok(Payment {
            id: payment_id,
            order_id: Uuid::new_v4(),
            user_id,
            payment_method: PaymentMethod::CreditCard,
            status: PaymentStatus::Cancelled,
            amount: rust_decimal::Decimal::ZERO,
            currency: Currency::USD,
            fee_amount: rust_decimal::Decimal::ZERO,
            net_amount: rust_decimal::Decimal::ZERO,
            provider_payment_id: None,
            provider_name: None,
            transaction_hash: None,
            failure_reason: Some("Cancelled by user".to_string()),
            metadata: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            completed_at: None,
            expires_at: None,
        })
    }
    
    pub async fn refund_payment(
        &self,
        payment_id: Uuid,
        req: &RefundPaymentRequest,
        user_id: Uuid,
    ) -> Result<PaymentRefund> {
        let payment = self.get_payment_by_id(payment_id, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("Payment not found"))?;
        
        if payment.status != PaymentStatus::Completed {
            return Err(anyhow::anyhow!("Payment cannot be refunded"));
        }
        
        let refund_amount = req.amount.unwrap_or(payment.amount);
        
        let refund = PaymentRefund {
            id: Uuid::new_v4(),
            payment_id,
            refund_amount,
            reason: req.reason.clone(),
            status: crate::models::payment::RefundStatus::Pending,
            provider_refund_id: None,
            transaction_hash: None,
            processed_by: user_id,
            created_at: chrono::Utc::now(),
            processed_at: None,
            metadata: req.metadata.clone(),
        };
        
        self.db.create_refund(&refund).await?;
        
        Ok(refund)
    }
    
    pub async fn get_user_payments(
        &self,
        user_id: Uuid,
        query: &crate::handlers::payment_handler::PaymentListQuery,
    ) -> Result<Vec<Payment>> {
        self.db.get_user_payments(user_id, query).await
    }
    
    // 私有方法
    async fn validate_order(&self, order_id: &Uuid, user_id: Uuid) -> Result<()> {
        // 调用订单服务验证订单
        // 这里简化处理
        Ok(())
    }
    
    fn calculate_fee(&self, payment_method: &PaymentMethod, amount: &rust_decimal::Decimal) -> rust_decimal::Decimal {
        use rust_decimal::Decimal;
        match payment_method {
            PaymentMethod::Solana => amount * Decimal::new(1, 3), // 0.1%
            PaymentMethod::CreditCard => amount * Decimal::new(29, 3) + Decimal::new(30, 2), // 2.9% + $0.30
            PaymentMethod::PayPal => amount * Decimal::new(29, 3) + Decimal::new(30, 2), // 2.9% + $0.30
            PaymentMethod::BankTransfer => Decimal::new(500, 2), // $5.00
            _ => Decimal::new(0, 0),
        }
    }
    
    async fn process_solana_payment(
        &self,
        payment: &mut Payment,
        req: &ProcessPaymentRequest,
    ) -> Result<()> {
        // 处理Solana支付
        if let Some(signature) = req.payment_method_details.get("signature") {
            let sig_str = signature.as_str().unwrap();
            if self.providers.solana.verify_transaction(sig_str).await? {
                payment.status = PaymentStatus::Completed;
                payment.transaction_hash = Some(sig_str.to_string());
                payment.completed_at = Some(chrono::Utc::now());
            } else {
                payment.status = PaymentStatus::Failed;
                payment.failure_reason = Some("Transaction verification failed".to_string());
            }
        }
        
        payment.updated_at = chrono::Utc::now();
        self.db.update_payment(payment).await?;
        Ok(())
    }
    
    async fn process_credit_card_payment(
        &self,
        payment: &mut Payment,
        req: &ProcessPaymentRequest,
    ) -> Result<()> {
        // 处理信用卡支付（通过Stripe）
        // 这里需要实现Stripe集成
        payment.status = PaymentStatus::Completed;
        payment.completed_at = Some(chrono::Utc::now());
        payment.updated_at = chrono::Utc::now();
        self.db.update_payment(payment).await?;
        Ok(())
    }
    
    async fn process_paypal_payment(
        &self,
        payment: &mut Payment,
        req: &ProcessPaymentRequest,
    ) -> Result<()> {
        // 处理PayPal支付
        payment.status = PaymentStatus::Completed;
        payment.completed_at = Some(chrono::Utc::now());
        payment.updated_at = chrono::Utc::now();
        self.db.update_payment(payment).await?;
        Ok(())
    }
    
    pub async fn handle_stripe_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<()> {
        // TODO: 实现 Stripe webhook 处理
        tracing::info!("Handling Stripe webhook");
        Ok(())
    }
    
    pub async fn handle_paypal_webhook(
        &self,
        payload: &str,
        headers: &actix_web::http::header::HeaderMap,
    ) -> Result<()> {
        // TODO: 实现 PayPal webhook 处理
        tracing::info!("Handling PayPal webhook");
        Ok(())
    }
} 