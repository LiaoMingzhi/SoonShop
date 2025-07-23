use std::sync::Arc;
use anyhow::Result;

use crate::config::{AppConfig, SolanaConfig, StripeConfig, PayPalConfig, BankTransferConfig};

pub mod solana;
pub mod stripe;
pub mod paypal;
pub mod bank_transfer;

use solana::SolanaProvider;
use stripe::StripeProvider;
use paypal::PayPalProvider;
use bank_transfer::BankTransferProvider;

// 注意：SolanaProvider 不能 Clone，所以我们使用 Arc
pub struct PaymentProviders {
    pub solana: Arc<solana::SolanaProvider>,
    pub stripe: stripe::StripeProvider,
    pub paypal: paypal::PayPalProvider,
    pub bank_transfer: bank_transfer::BankTransferProvider,
}

impl PaymentProviders {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        let solana = Arc::new(solana::SolanaProvider::new(&config.solana).await?);
        let stripe = stripe::StripeProvider::new(&config.stripe).await?;
        let paypal = paypal::PayPalProvider::new(&config.paypal).await?;
        let bank_transfer = bank_transfer::BankTransferProvider::new(&config.bank_transfer).await?;

        Ok(Self {
            solana,
            stripe,
            paypal,
            bank_transfer,
        })
    }
    
    pub async fn health_check(&self) -> Result<()> {
        // 检查所有支付提供商的健康状态
        self.solana.health_check().await?;
        self.stripe.health_check().await?;
        self.paypal.health_check().await?;
        self.bank_transfer.health_check().await?;
        
        Ok(())
    }
}

impl Clone for PaymentProviders {
    fn clone(&self) -> Self {
        Self {
            solana: self.solana.clone(),
            stripe: self.stripe.clone(),
            paypal: self.paypal.clone(),
            bank_transfer: self.bank_transfer.clone(),
        }
    }
} 