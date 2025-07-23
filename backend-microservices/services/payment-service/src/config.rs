use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub rabbitmq_url: String,
    pub jwt_secret: String,
    
    // 外部服务
    pub order_service_url: String,
    pub user_service_url: String,
    pub notification_service_url: String,
    
    // Solana配置
    pub solana: SolanaConfig,
    
    // 支付提供商配置
    pub stripe: StripeConfig,
    pub paypal: PayPalConfig,
    pub bank_transfer: BankTransferConfig,
    
    // 安全配置
    pub encryption_key: String,
    pub webhook_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub keypair_path: String,
    pub program_id: String,
    pub token_mint: String,
    pub treasury_account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeConfig {
    pub api_key: String,
    pub webhook_endpoint: String,
    pub webhook_secret: String,
    pub currency: String,
    pub sandbox: bool,
    pub return_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalConfig {
    pub client_id: String,
    pub client_secret: String,
    pub sandbox: bool,
    pub webhook_id: String,
    pub return_url: String,
    pub cancel_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferConfig {
    pub bank_name: String,
    pub account_number: String,
    pub routing_number: String,
    pub swift_code: String,
    pub max_transfer_amount: f64,
    pub min_transfer_amount: f64,
    pub processing_days: u32,
    pub max_batch_size: usize,
    pub reference_prefix: String,
    pub supported_currencies: Vec<String>,
    pub fee_percentage: f64,
    pub fixed_fee: f64,
    pub max_fee: f64,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(AppConfig {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8004".to_string())
                .parse()
                .unwrap_or(8004),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:password@localhost/payment_service".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            rabbitmq_url: env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string()),
            
            order_service_url: env::var("ORDER_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8003".to_string()),
            user_service_url: env::var("USER_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8001".to_string()),
            notification_service_url: env::var("NOTIFICATION_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8008".to_string()),
            
            solana: SolanaConfig {
                rpc_url: env::var("SOLANA_RPC_URL")
                    .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
                ws_url: env::var("SOLANA_WS_URL")
                    .unwrap_or_else(|_| "wss://api.devnet.solana.com".to_string()),
                keypair_path: env::var("SOLANA_KEYPAIR_PATH")
                    .unwrap_or_else(|_| "./keypair.json".to_string()),
                program_id: env::var("SOLANA_PROGRAM_ID")
                    .unwrap_or_else(|_| "SoonShop1111111111111111111111111111111111111".to_string()),
                token_mint: env::var("SOLANA_TOKEN_MINT")
                    .unwrap_or_else(|_| "So11111111111111111111111111111111111111112".to_string()),
                treasury_account: env::var("SOLANA_TREASURY_ACCOUNT")
                    .unwrap_or_else(|_| "".to_string()),
            },
            
            stripe: StripeConfig {
                api_key: env::var("STRIPE_API_KEY")
                    .unwrap_or_else(|_| "sk_test_...".to_string()),
                webhook_endpoint: env::var("STRIPE_WEBHOOK_ENDPOINT")
                    .unwrap_or_else(|_| "/webhooks/stripe".to_string()),
                webhook_secret: env::var("STRIPE_WEBHOOK_SECRET")
                    .unwrap_or_else(|_| "whsec_...".to_string()),
                currency: env::var("STRIPE_CURRENCY")
                    .unwrap_or_else(|_| "usd".to_string()),
                sandbox: env::var("STRIPE_SANDBOX")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                return_url: env::var("STRIPE_RETURN_URL")
                    .unwrap_or_else(|_| "http://localhost:3000/payment/success".to_string()),
            },
            
            paypal: PayPalConfig {
                client_id: env::var("PAYPAL_CLIENT_ID")
                    .unwrap_or_else(|_| "".to_string()),
                client_secret: env::var("PAYPAL_CLIENT_SECRET")
                    .unwrap_or_else(|_| "".to_string()),
                sandbox: env::var("PAYPAL_SANDBOX")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                webhook_id: env::var("PAYPAL_WEBHOOK_ID")
                    .unwrap_or_else(|_| "".to_string()),
                return_url: env::var("PAYPAL_RETURN_URL")
                    .unwrap_or_else(|_| "http://localhost:3000/payment/success".to_string()),
                cancel_url: env::var("PAYPAL_CANCEL_URL")
                    .unwrap_or_else(|_| "http://localhost:3000/payment/cancel".to_string()),
            },
            
            bank_transfer: BankTransferConfig {
                bank_name: env::var("BANK_NAME")
                    .unwrap_or_else(|_| "SoonShop Bank".to_string()),
                account_number: env::var("BANK_ACCOUNT_NUMBER")
                    .unwrap_or_else(|_| "1234567890".to_string()),
                routing_number: env::var("BANK_ROUTING_NUMBER")
                    .unwrap_or_else(|_| "123456789".to_string()),
                swift_code: env::var("BANK_SWIFT_CODE")
                    .unwrap_or_else(|_| "SOONSHOP".to_string()),
                max_transfer_amount: env::var("BANK_MAX_TRANSFER_AMOUNT")
                    .unwrap_or_else(|_| "100000.0".to_string())
                    .parse()
                    .unwrap_or(100000.0),
                min_transfer_amount: env::var("BANK_MIN_TRANSFER_AMOUNT")
                    .unwrap_or_else(|_| "1.0".to_string())
                    .parse()
                    .unwrap_or(1.0),
                processing_days: env::var("BANK_PROCESSING_DAYS")
                    .unwrap_or_else(|_| "3".to_string())
                    .parse()
                    .unwrap_or(3),
                max_batch_size: env::var("BANK_MAX_BATCH_SIZE")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .unwrap_or(100),
                reference_prefix: env::var("BANK_REFERENCE_PREFIX")
                    .unwrap_or_else(|_| "SOONSHOP".to_string()),
                supported_currencies: vec!["USD".to_string(), "EUR".to_string()],
                fee_percentage: env::var("BANK_FEE_PERCENTAGE")
                    .unwrap_or_else(|_| "2.5".to_string())
                    .parse()
                    .unwrap_or(2.5),
                fixed_fee: env::var("BANK_FIXED_FEE")
                    .unwrap_or_else(|_| "0.30".to_string())
                    .parse()
                    .unwrap_or(0.30),
                max_fee: env::var("BANK_MAX_FEE")
                    .unwrap_or_else(|_| "25.0".to_string())
                    .parse()
                    .unwrap_or(25.0),
            },
            
            encryption_key: env::var("ENCRYPTION_KEY")
                .unwrap_or_else(|_| "your-encryption-key-32-bytes-long".to_string()),
            webhook_secret: env::var("WEBHOOK_SECRET")
                .unwrap_or_else(|_| "your-webhook-secret".to_string()),
        })
    }
} 