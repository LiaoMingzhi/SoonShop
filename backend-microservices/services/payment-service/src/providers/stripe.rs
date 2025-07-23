use std::collections::HashMap;
use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION}};
use uuid::Uuid;
use tracing::{info, error, warn, instrument};

use crate::config::StripeConfig;
use crate::models::{Payment, PaymentStatus, PaymentIntent};
use crate::error::PaymentError;

/// Stripe支付提供商
#[derive(Clone)]
pub struct StripeProvider {
    client: Client,
    config: StripeConfig,
    base_url: String,
}

/// Stripe支付意图创建请求
#[derive(Serialize, Debug)]
struct CreatePaymentIntentRequest {
    amount: u64,
    currency: String,
    automatic_payment_methods: AutomaticPaymentMethods,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
}

#[derive(Serialize, Debug)]
struct AutomaticPaymentMethods {
    enabled: bool,
}

/// Stripe支付意图响应
#[derive(Deserialize, Debug)]
pub struct StripePaymentIntent {
    pub id: String,
    pub amount: u64,
    pub currency: String,
    pub status: String,
    pub client_secret: Option<String>,
    pub payment_method: Option<String>,
    pub description: Option<String>,
    pub receipt_email: Option<String>,
    pub created: u64,
    pub metadata: HashMap<String, String>,
    pub charges: Option<StripeCharges>,
    pub last_payment_error: Option<StripeError>,
}

#[derive(Deserialize, Debug)]
pub struct StripeCharges {
    pub data: Vec<StripeCharge>,
}

#[derive(Deserialize, Debug)]
pub struct StripeCharge {
    pub id: String,
    pub amount: u64,
    pub currency: String,
    pub status: String,
    pub paid: bool,
    pub refunded: bool,
    pub payment_method_details: Option<Value>,
    pub receipt_url: Option<String>,
    pub billing_details: Option<Value>,
    pub outcome: Option<StripeOutcome>,
}

#[derive(Deserialize, Debug)]
pub struct StripeOutcome {
    pub network_status: Option<String>,
    pub reason: Option<String>,
    pub risk_level: Option<String>,
    pub risk_score: Option<u32>,
    #[serde(rename = "type")]
    pub outcome_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct StripeError {
    pub code: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub decline_code: Option<String>,
}

/// Stripe退款请求
#[derive(Serialize, Debug)]
struct RefundRequest {
    charge: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

/// Stripe退款响应
#[derive(Deserialize, Debug)]
pub struct StripeRefund {
    pub id: String,
    pub amount: u64,
    pub currency: String,
    pub charge: String,
    pub status: String,
    pub reason: Option<String>,
    pub created: u64,
    pub metadata: HashMap<String, String>,
}

/// Stripe客户创建请求
#[derive(Serialize, Debug)]
struct CreateCustomerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

/// Stripe客户响应
#[derive(Deserialize, Debug)]
pub struct StripeCustomer {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created: u64,
    pub metadata: HashMap<String, String>,
}

impl StripeProvider {
    /// 创建新的Stripe支付提供商实例
    pub async fn new(config: &StripeConfig) -> Result<Self> {
        let mut headers = HeaderMap::new();
        let auth_header = format!("Bearer {}", config.api_key);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        let base_url = if config.sandbox {
            "https://api.stripe.com".to_string()
        } else {
            "https://api.stripe.com".to_string()
        };

        Ok(StripeProvider {
            client,
            config: config.clone(),
            base_url,
        })
    }

    /// 创建支付意图
    #[instrument(skip(self))]
    pub async fn create_payment_intent(
        &self,
        amount: u64,
        currency: &str,
        customer_email: Option<&str>,
        description: Option<&str>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<StripePaymentIntent> {
        let url = format!("{}/v1/payment_intents", self.base_url);
        
        let request = CreatePaymentIntentRequest {
            amount,
            currency: currency.to_lowercase(),
            automatic_payment_methods: AutomaticPaymentMethods { enabled: true },
            customer: None, // 可以先创建客户再设置
            description: description.map(|s| s.to_string()),
            metadata,
            receipt_email: customer_email.map(|s| s.to_string()),
            payment_method: None,
            confirm: None,
            return_url: Some(self.config.return_url.clone()),
        };

        info!("Creating Stripe payment intent for amount: {} {}", amount, currency);

        let form_data = serde_urlencoded::to_string(&request)
            .map_err(|e| PaymentError::InvalidRequest(format!("Failed to serialize request: {}", e)))?;

        let response = self.client
            .post(&url)
            .body(form_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Stripe API error: {}", error_text);
            bail!(PaymentError::ProviderError(format!("Stripe error: {}", error_text)));
        }

        let payment_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        info!("Successfully created Stripe payment intent: {}", payment_intent.id);
        Ok(payment_intent)
    }

    /// 确认支付意图
    #[instrument(skip(self))]
    pub async fn confirm_payment_intent(
        &self,
        payment_intent_id: &str,
        payment_method: Option<&str>,
    ) -> Result<StripePaymentIntent> {
        let url = format!("{}/v1/payment_intents/{}/confirm", self.base_url, payment_intent_id);
        
        let mut form_data = String::new();
        if let Some(pm) = payment_method {
            form_data.push_str(&format!("payment_method={}", pm));
        }

        info!("Confirming Stripe payment intent: {}", payment_intent_id);

        let response = self.client
            .post(&url)
            .body(form_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Stripe confirm error: {}", error_text);
            bail!(PaymentError::ProviderError(format!("Stripe confirm error: {}", error_text)));
        }

        let payment_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        info!("Successfully confirmed Stripe payment intent: {}", payment_intent.id);
        Ok(payment_intent)
    }

    /// 获取支付意图详情
    #[instrument(skip(self))]
    pub async fn get_payment_intent(&self, payment_intent_id: &str) -> Result<StripePaymentIntent> {
        let url = format!("{}/v1/payment_intents/{}", self.base_url, payment_intent_id);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Stripe API error: {}", error_text);
            bail!(PaymentError::ProviderError(format!("Stripe error: {}", error_text)));
        }

        let payment_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        Ok(payment_intent)
    }

    /// 创建退款
    #[instrument(skip(self))]
    pub async fn create_refund(
        &self,
        charge_id: &str,
        amount: Option<u64>,
        reason: Option<&str>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<StripeRefund> {
        let url = format!("{}/v1/refunds", self.base_url);
        
        let request = RefundRequest {
            charge: charge_id.to_string(),
            amount,
            reason: reason.map(|s| s.to_string()),
            metadata,
        };

        info!("Creating Stripe refund for charge: {}", charge_id);

        let form_data = serde_urlencoded::to_string(&request)
            .map_err(|e| PaymentError::InvalidRequest(format!("Failed to serialize request: {}", e)))?;

        let response = self.client
            .post(&url)
            .body(form_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Stripe refund error: {}", error_text);
            bail!(PaymentError::ProviderError(format!("Stripe refund error: {}", error_text)));
        }

        let refund: StripeRefund = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        info!("Successfully created Stripe refund: {}", refund.id);
        Ok(refund)
    }

    /// 创建客户
    #[instrument(skip(self))]
    pub async fn create_customer(
        &self,
        email: Option<&str>,
        name: Option<&str>,
        description: Option<&str>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<StripeCustomer> {
        let url = format!("{}/v1/customers", self.base_url);
        
        let request = CreateCustomerRequest {
            email: email.map(|s| s.to_string()),
            name: name.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            metadata,
        };

        info!("Creating Stripe customer");

        let form_data = serde_urlencoded::to_string(&request)
            .map_err(|e| PaymentError::InvalidRequest(format!("Failed to serialize request: {}", e)))?;

        let response = self.client
            .post(&url)
            .body(form_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Stripe customer creation error: {}", error_text);
            bail!(PaymentError::ProviderError(format!("Stripe customer error: {}", error_text)));
        }

        let customer: StripeCustomer = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        info!("Successfully created Stripe customer: {}", customer.id);
        Ok(customer)
    }

    /// 处理Webhook事件
    #[instrument(skip(self))]
    pub async fn handle_webhook(&self, payload: &str, signature: &str) -> Result<Value> {
        // 验证webhook签名
        self.verify_webhook_signature(payload, signature)?;

        // 解析事件
        let event: Value = serde_json::from_str(payload)
            .map_err(|e| PaymentError::InvalidRequest(format!("Invalid webhook payload: {}", e)))?;

        let event_type = event["type"].as_str()
            .ok_or(PaymentError::InvalidRequest("Missing event type".to_string()))?;

        info!("Processing Stripe webhook event: {}", event_type);

        match event_type {
            "payment_intent.succeeded" => {
                info!("Payment intent succeeded");
            }
            "payment_intent.payment_failed" => {
                warn!("Payment intent failed");
            }
            "charge.dispute.created" => {
                warn!("Charge dispute created");
            }
            _ => {
                info!("Unhandled webhook event type: {}", event_type);
            }
        }

        Ok(event)
    }

    /// 验证webhook签名
    fn verify_webhook_signature(&self, payload: &str, signature: &str) -> Result<()> {
        // TODO: 实现HMAC-SHA256签名验证
        // 这里应该使用webhook_secret来验证签名
        
        if self.config.webhook_secret.is_empty() {
            warn!("Webhook secret not configured, skipping signature verification");
            return Ok(());
        }

        // 暂时返回成功，实际应该实现签名验证
        Ok(())
    }

    /// 健康检查
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> Result<()> {
        let url = format!("{}/v1/charges?limit=1", self.base_url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Health check failed: {}", e)))?;

        if response.status().is_success() {
            info!("Stripe health check passed");
            Ok(())
        } else {
            error!("Stripe health check failed with status: {}", response.status());
            bail!(PaymentError::ProviderError("Stripe health check failed".to_string()));
        }
    }

    /// 将Stripe状态转换为内部支付状态
    pub fn map_stripe_status_to_payment_status(stripe_status: &str) -> PaymentStatus {
        match stripe_status {
            "requires_payment_method" => PaymentStatus::Pending,
            "requires_confirmation" => PaymentStatus::Pending,
            "requires_action" => PaymentStatus::Pending,
            "processing" => PaymentStatus::Processing,
            "succeeded" => PaymentStatus::Completed,
            "canceled" => PaymentStatus::Cancelled,
            "requires_capture" => PaymentStatus::Processing,
            _ => PaymentStatus::Failed,
        }
    }

    /// 格式化金额（转换为最小货币单位）
    pub fn format_amount(amount: f64, currency: &str) -> u64 {
        match currency.to_uppercase().as_str() {
            "JPY" | "KRW" | "VND" => amount as u64, // 这些货币没有小数位
            _ => (amount * 100.0) as u64, // 大多数货币转换为分
        }
    }

    /// 解析金额（从最小货币单位转换）
    pub fn parse_amount(amount: u64, currency: &str) -> f64 {
        match currency.to_uppercase().as_str() {
            "JPY" | "KRW" | "VND" => amount as f64,
            _ => amount as f64 / 100.0,
        }
    }
} 